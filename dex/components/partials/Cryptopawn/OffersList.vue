<script setup lang="ts">
import { spotQuantityFromChainQuantityToFixed } from '@injectivelabs/sdk-ts'
import { PropType } from '~~/.nuxt/imports'
import { Modal } from '~~/types'
import { Proposition, PropositionState, PropositionType } from '~~/types/cryptopawn'

const props = defineProps({
  listType: {
    type: String
  },
  offerType: {
    type: String as PropType<PropositionType>,
    required: true
  },
  offers: {
    type: Array as PropType<Proposition[]>,
    default: []
  }
})

const tokens = useTokenStore()
const wallet = useWalletStore()
const pawn = useCryptoPawnStore()
const modal = useModalStore()

const computedOffers = computed(() => {
  return props.offers.map((offer) => {
    const assetsToken = tokens.tokens.find((token) => token.denom === offer.assets.denom)
    const depositToken = tokens.tokens.find((token) => token.denom === offer.deposit.denom)
    const premiumToken = tokens.tokens.find((token) => token.denom === offer.premium.denom)

    return {
      ...offer,
      assets: {
        amount: spotQuantityFromChainQuantityToFixed({
          value: offer.assets.amount,
          baseDecimals: assetsToken?.decimals
        }),
        denom: assetsToken?.symbol,
        img: assetsToken?.logo
      },
      deposit: {
        amount: spotQuantityFromChainQuantityToFixed({
          value: offer.deposit.amount,
          baseDecimals: depositToken?.decimals
        }),
        denom: depositToken?.symbol,
        img: depositToken?.logo
      },
      premium: {
        amount: spotQuantityFromChainQuantityToFixed({
          value: offer.premium.amount,
          baseDecimals: premiumToken?.decimals
        }),
        denom: premiumToken?.symbol,
        img: premiumToken?.logo
      },
      expiry: props.listType !== 'Market' ? parseLeftTime(offer.expiry) : parseExpiryDate(offer.expiry),
      period: parsePeriod(offer.period),
      action: getAction(offer)
    }
  })
})

const monthNames = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']

const parseExpiryDate = (timestamp: number) => {
  const expiryDate = timestamp * 1000
  const date = new Date(expiryDate)
  return isExpired(expiryDate) ? 'Expired' : date.getDate() + ' ' + monthNames[date.getMonth()] + ' ' + date.getFullYear()
}

const parseLeftTime = (timestamp: number) => {
  const expiryDate = timestamp * 1000
  const now = new Date().getTime()
  const diff = timestamp - now / 1000
  return isExpired(expiryDate) ? 'Expired' : parseTimestamp(diff.toString())
}

const parsePeriod = (timestamp: number) => {
  return timestamp / 3600 + ' days'
}

const parseTimestamp = (timestamp: string) => {
  const timestampNumber = parseInt(timestamp)

  if (timestampNumber < 60) {
    return Math.ceil(timestampNumber) + ' s'
  }

  if (timestampNumber < 60 * 60) {
    return Math.ceil(timestampNumber / 60) + ' min'
  }

  if (timestampNumber < 24 * 60 * 60) {
    return Math.ceil(timestampNumber / (60 * 60)) + ' hours'
  }

  return Math.ceil(timestampNumber / (60 * 60 * 24)) + ' days'
}

enum ACTIONS {
  REJECT = 'Cancel',
  CLOSE = 'Close',
  ACCEPT = 'Accept',
  REJECTED = 'Canceled',
  CLOSED = 'Closed',
  ACCEPTED = 'Accepted',
  UNKNOWN = 'Unknown action'
}

const actionsHandlers: Record<ACTIONS, (offer: any) => void> = {
  [ACTIONS.CLOSE]: (offer) => {
    modal.openModal({ type: Modal.OfferDetailsCryptoPawn, data: { offer } })
  },
  [ACTIONS.ACCEPT]: (offer) => {
    modal.openModal({ type: Modal.OfferDetailsCryptoPawn, data: { offer } })
  },
  [ACTIONS.REJECT]: async (offer) => {
    await pawn.rejectProposition(offer.id)
    pawn.fetchPropositions()
  },
  [ACTIONS.REJECTED]: () => {
    return
  },
  [ACTIONS.CLOSED]: () => {
    return
  },
  [ACTIONS.ACCEPTED]: () => {
    return
  },
  [ACTIONS.UNKNOWN]: () => {
    return
  }
}

const getAction = (offer: Proposition) => {
  if (offer.owner === wallet.injectiveAddress && offer.state === 'active') {
    return ACTIONS.REJECT
  }

  if (isClosable(offer)) {
    return ACTIONS.CLOSE
  }

  if (offer.state === 'active') {
    return ACTIONS.ACCEPT
  }

  if (offer.state === 'rejected') {
    return ACTIONS.REJECTED
  }

  if (offer.state === 'accepted') {
    return ACTIONS.ACCEPTED
  }

  if (offer.state === 'closed') {
    return ACTIONS.CLOSED
  }

  return ACTIONS.UNKNOWN
}

const isInActive = (offer: any) => {
  if (offer.action === ACTIONS.ACCEPTED) {
    return true
  }

  if (offer.action === ACTIONS.REJECTED) {
    return true
  }

  if (offer.action === ACTIONS.CLOSED) {
    return true
  }

  return false
}

const isClosable = (offer: Proposition) => {
  const isOwner = offer.owner === wallet.injectiveAddress
  const isContractor = offer.contractor === wallet.injectiveAddress
  const is_expired = isExpired(offer.expiry * 1000)
  const type = offer.proposition_type
  const state = offer.state

  if (state !== PropositionState.Accepted) {
    return false
  }

  if (type === 'ask' && !is_expired && isOwner) {
    return true
  }

  if (type === 'bid' && is_expired && isOwner) {
    return true
  }

  if (type === 'ask' && is_expired && isContractor) {
    return true
  }

  if (type === 'bid' && !is_expired && isContractor) {
    return true
  }

  return false
}

const isExpired = (timestamp: number) => {
  const now = new Date().getTime()

  if (timestamp < now) {
    return true
  }

  return false
}

const handleOpenNewOfferModal = () => {
  modal.openModal({ type: Modal.CreateOfferCryptoPawn, data: { type: props.offerType } })
}
</script>

<template>
  <div class="bg-qwerty-shade2 p-10 rounded-2xl">
    <div class="flex justify-between">
      <slot name="title" />
      <AppButton @click="handleOpenNewOfferModal" class="bg-qwerty-primary text-qwerty-background font-semibold"> New offer + </AppButton>
    </div>
    <table class="w-full border-collapse hidden lg:table">
      <thead>
        <slot name="tableHeader" />
      </thead>
      <tbody>
        <tr
          v-for="offer in computedOffers"
          class="border-b border-gray-600 last-of-type:border-b-transparent text-qw bg-transparent py-0 overflow-hidden h-14 gap-2 transition-all cursor-pointer"
        >
          <td>
            <div class="flex justify-start items-center gap-2">
              <img class="w-[18px]" :src="'/vendor/@injectivelabs/token-metadata/' + offer.assets.img" />
              <span class="text-qwerty-white font-bold tracking-wide text-xs lg:text-sm uppercase">
                {{ offer.assets.amount }} {{ offer.assets.denom }}
              </span>
            </div>
          </td>
          <td>
            <div class="flex justify-center items-center gap-2">
              <img class="w-[18px]" :src="'/vendor/@injectivelabs/token-metadata/' + offer.deposit.img" />
              <span class="text-qwerty-white font-bold tracking-wide text-xs lg:text-sm uppercase">
                {{ offer.deposit.amount }} {{ offer.deposit.denom }}
              </span>
            </div>
          </td>
          <td>
            <div class="flex justify-center items-center gap-2">
              <img class="w-[18px]" :src="'/vendor/@injectivelabs/token-metadata/' + offer.premium.img" />
              <span class="text-qwerty-white font-bold tracking-wide text-xs lg:text-sm uppercase">
                {{ offer.premium.amount }} {{ offer.premium.denom }}
              </span>
            </div>
          </td>
          <td>
            <div class="flex justify-center items-center gap-2">
              <span class="text-qwerty-white font-bold tracking-wide text-xs lg:text-sm uppercase"> {{ offer.period }} </span>
            </div>
          </td>
          <td>
            <div class="flex justify-center items-center gap-2">
              <span class="text-qwerty-white font-bold tracking-wide text-xs lg:text-sm uppercase"> {{ offer.expiry }} </span>
            </div>
          </td>
          <td>
            <div class="flex justify-end items-center gap-2">
              <span
                @click="() => actionsHandlers[offer.action](offer)"
                :class="isInActive(offer) ? 'text-qwerty-secondary2' : 'text-qwerty-primary'"
                class="tracking-wide text-xs lg:text-sm"
              >
                {{ offer.action }}
              </span>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
