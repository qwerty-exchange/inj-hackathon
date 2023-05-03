import { MsgExecuteContractCompat, spotQuantityToChainQuantityToFixed, spotQuantityToChainQuantity } from '@injectivelabs/sdk-ts'
import { Coin } from '@injectivelabs/ts-types'
import { BigNumber } from '@injectivelabs/utils'
import { msgBroadcastClient, wasmApi } from '~~/app/Services'
import { WalletStoreState } from '~~/store/wallet'
import { Proposition, PropositionState, PropositionType } from '~~/types/cryptopawn'

const contractAddress = 'inj1ntkgeswdwx6jhg0q7n9rtunqxsvnsnf5k5vvm8'
const EMPTY_FUNDS = [{ amount: '0', denom: 'inj' }]

export const useCryptoPawnStore = defineStore('cryptopawn', {
  state: () => ({ offers: [] }),
  getters: {
    getOffers: ({ offers }: { offers: Proposition[] }) => {
      return offers
    }
  },
  actions: {
    async fetchPropositions() {
      const payload = {
        get_propositions: {
          // start_before: start_before,
          limit: 100
        }
      }

      const query = Buffer.from(JSON.stringify(payload)).toString('base64')
      const response = await wasmApi.fetchSmartContractState(contractAddress, query)
      const data = JSON.parse(Buffer.from(response.data).toString())


      this.$patch({
        offers: [...data.map((x: any) => ({ id: x[0], ...x[1] }))]
      })
    },

    createProposition: async (formData: Proposition) => {
      const wallet = useWalletStore()
      const newOffer = toRaw(formData)

      const { isAsk, assetsFundsToChain, collateralFundsToChain, getData, fundsToChain } = offerService(newOffer)

      const funds = isAsk() ? collateralFundsToChain() : assetsFundsToChain()
      const data = {
        ...getData(),
        ...fundsToChain()
      }

      await chainService(wallet).createProposition(data, funds)
    },

    rejectProposition: async (proposition_id: string) => {
      const wallet = useWalletStore()
      await chainService(wallet).rejectProposition(proposition_id)
    },

    acceptProposition: async (proposition_id: string) => {
      const wallet = useWalletStore()
      const store = useCryptoPawnStore()
      const rowOffer = toRaw(store.getOffers.find((item) => item.id === proposition_id)!)

      const { isAsk, assetsFunds, collateralFunds } = offerService(rowOffer)
      const funds = isAsk() ? assetsFunds() : collateralFunds()

      await chainService(wallet).acceptProposition(rowOffer.id!, funds)
    },

    closeProposition: async (proposition_id: string) => {
      const wallet = useWalletStore()
      const store = useCryptoPawnStore()
      const rowOffer = toRaw(store.getOffers.find((item) => item.id === proposition_id)!)

      const { getContractors, assetsFunds } = offerService(rowOffer)
      const { borrower } = getContractors()
      const isUserBorrower = wallet.injectiveAddress === borrower
      const funds = isUserBorrower ? assetsFunds() : EMPTY_FUNDS

      await chainService(wallet).closeProposition(rowOffer.id!, funds)
    }
  }
})

const offerService = (offer: Proposition) => {
  const now = Math.floor(new Date().getTime() / 1000)
  const day = 24 * 60 * 60
  const minute = 60
  offer.expiry = now + offer.expiry * day
  offer.period = offer.period * minute * 60

  return {
    getData() {
      return offer
    },
    getContractors() {
      const { owner, contractor, proposition_type } = offer

      return {
        owner,
        contractor,
        borrower: proposition_type === PropositionType.Ask ? owner : contractor,
        lender: proposition_type === PropositionType.Bid ? owner : contractor
      }
    },
    isBid() {
      return offer.proposition_type === PropositionType.Bid
    },
    isAsk() {
      return offer.proposition_type === PropositionType.Ask
    },
    isExpired() {
      const now = new Date().getTime() / 1000

      return now > offer.expiry
    },
    fundsToChain() {
      const { assets, deposit, premium } = offer
      return {
        assets: {
          amount: transformSpotQuantityToChainQuantityToFixed(assets),
          denom: assets.denom
        },
        deposit: {
          amount: transformSpotQuantityToChainQuantityToFixed(deposit),
          denom: deposit.denom
        },
        premium: {
          amount: transformSpotQuantityToChainQuantityToFixed(premium),
          denom: premium.denom
        }
      }
    },
    assetsFundsToChain() {
      const { assets } = offer
      return transformFunds({
        [assets.denom]: transformSpotQuantityToChainQuantity(assets)
      })
    },
    collateralFundsToChain() {
      const { premium, deposit } = offer

      const funds: Record<string, BigNumber> = {}

      Object.assign(funds, { [premium.denom]: new BigNumber(0) })
      Object.assign(funds, { [deposit.denom]: new BigNumber(0) })

      funds[deposit.denom] = BigNumber.sum(transformSpotQuantityToChainQuantity(deposit), funds[deposit.denom])
      funds[premium.denom] = BigNumber.sum(transformSpotQuantityToChainQuantity(premium), funds[premium.denom])

      return transformFunds(funds)
    },
    assetsFunds() {
      const { assets } = offer
      return transformFunds({
        [assets.denom]: new BigNumber(assets.amount)
      })
    },
    collateralFunds() {
      const { premium, deposit } = offer

      const funds: Record<string, BigNumber> = {}

      Object.assign(funds, { [premium.denom]: new BigNumber(0) })
      Object.assign(funds, { [deposit.denom]: new BigNumber(0) })

      funds[deposit.denom] = BigNumber.sum(new BigNumber(deposit.amount), funds[deposit.denom])
      funds[premium.denom] = BigNumber.sum(new BigNumber(premium.amount), funds[premium.denom])

      return transformFunds(funds)
    }
  }
}

const chainService = (wallet: WalletStoreState) => {
  return {
    async acceptProposition(proposition_id: string, funds: Array<Coin>) {
      const message = MsgExecuteContractCompat.fromJSON({
        contractAddress,
        sender: wallet.injectiveAddress,
        funds,
        msg: {
          accept_proposition: {
            proposition_id
          }
        }
      })

      await broadcastMessage({
        senderAddress: wallet.injectiveAddress,
        message
      })
    },
    async closeProposition(proposition_id: string, funds: Array<Coin>) {
      const message = MsgExecuteContractCompat.fromJSON({
        contractAddress,
        sender: wallet.injectiveAddress,
        funds,
        msg: {
          close_proposition: {
            proposition_id
          }
        }
      })

      await broadcastMessage({
        senderAddress: wallet.injectiveAddress,
        message
      })
    },
    async createProposition(offer: Proposition, funds: Array<Coin>) {
      const message = MsgExecuteContractCompat.fromJSON({
        contractAddress,
        sender: wallet.injectiveAddress,
        funds,
        msg: {
          create_proposition: offer
        }
      })

      await broadcastMessage({
        senderAddress: wallet.injectiveAddress,
        message
      })
    },
    async rejectProposition(proposition_id: string) {
      const message = MsgExecuteContractCompat.fromJSON({
        contractAddress,
        sender: wallet.injectiveAddress,
        msg: {
          reject_proposition: {
            proposition_id
          }
        }
      })

      await broadcastMessage({
        senderAddress: wallet.injectiveAddress,
        message
      })
    }
  }
}

const broadcastMessage = ({ message, senderAddress }: { message: MsgExecuteContractCompat; senderAddress: string }) => {
  return msgBroadcastClient.broadcast({
    address: senderAddress,
    msgs: message
  })
}

const transformFunds = (funds: Record<string, BigNumber>) => {
  return Object.entries(funds).map(([denom, amount]) => {
    return {
      denom,
      amount: amount.toString()
    }
  })
}

const transformSpotQuantityToChainQuantity = ({ amount, denom }: { amount: string; denom: string }) => {
  const tokens = useTokenStore()

  return spotQuantityToChainQuantity({
    value: amount,
    baseDecimals: tokens.tokens.find((token) => token.denom === denom)?.decimals
  })
}

const transformSpotQuantityToChainQuantityToFixed = ({ amount, denom }: { amount: string; denom: string }) => {
  const tokens = useTokenStore()

  return spotQuantityToChainQuantityToFixed({
    value: amount,
    baseDecimals: tokens.tokens.find((token) => token.denom === denom)?.decimals
  })
}
