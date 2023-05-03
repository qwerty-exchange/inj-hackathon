<!--
  - ikonki tokenów

  - historyczne transakcje
  - refaktor (podzielić na mniejsze komponenty)
  - style tabelki, aby nie zmieniała się szerokość kolumn przy przełączaniu pomiędzy tabami
  - dodać loader na guziki w trakcie podpisywania transakcji
  - dodać loader na guziki w trakcie podpisywania notifi
  - lista na mobile

  - filtr po tokenach
  - odświeżanie czasu expiry
  - paginacja
  - sortowanie tabelki po kolumnach
  - AppSelectToken - pozwala tylko na wpisanie max tego co mamy w portfelu (źle)
  - Na jednym koncie mogę tworzyć oferty, a na drugim nie (keplr)
  - handlowanie akcji jeżeli nie podłączony wallet
  - modal przy akcjach transakcyjnych
  - po zamknięciu modala "Create transaction" zerują się inputy, ale state nie i nadal można submitować form
  - pomyśleć nad użyciem hooka do przechowywania state forma create proposition
  - zaawansowane filtry
 -->

<script lang="ts" setup>
import { Proposition, PropositionState, PropositionType } from '~~/types/cryptopawn'
import { Modal } from '~~/types'

const pawn = useCryptoPawnStore()
const wallet = useWalletStore()
const modal = useModalStore()

const activeType = ref('Market')

const offers = ref<Array<Proposition>>([])

const getUserPositions = (offers: Proposition[]) => {
  return offers.filter(
    (item) => (item.owner === wallet.injectiveAddress || item.contractor === wallet.injectiveAddress) && item.state === PropositionState.Accepted
  )
}

const getUserOffers = (offers: Proposition[]) => {
  return offers.filter((item) => item.owner === wallet.injectiveAddress && item.state === PropositionState.Active)
}

const getMarketOffers = (offers: Proposition[]) => {
  return offers.filter((offer) => offer.state === PropositionState.Active && !isExpired(offer.expiry * 1000))
}

const isExpired = (timestamp: number) => {
  const now = new Date().getTime()

  return timestamp < now ? true : false
}

const getFilteredOffers = () => {
  if (activeType.value === 'Market') {
    return getMarketOffers(pawn.getOffers)
  }

  if (activeType.value === 'Positions') {
    return getUserPositions(pawn.getOffers)
  }

  if (activeType.value === 'Offers') {
    return getUserOffers(pawn.getOffers)
  }
}

watch(activeType, () => {
  offers.value = getFilteredOffers()!
})

onMounted(async () => {
  await pawn.fetchPropositions()
  offers.value = getMarketOffers(pawn.getOffers)
})

watch(
  () => pawn.getOffers,
  () => {
    offers.value = getFilteredOffers()!
  }
)

const lendOffers = computed(() => {
  return offers.value.filter(
    (offer) =>
      (offer.proposition_type === PropositionType.Ask && wallet.injectiveAddress !== offer.owner) ||
      (offer.proposition_type === PropositionType.Bid && wallet.injectiveAddress === offer.owner)
  )
})

const borrowOffers = computed(() => {
  return offers.value.filter(
    (offer) =>
      (offer.proposition_type === PropositionType.Bid && wallet.injectiveAddress !== offer.owner) ||
      (offer.proposition_type === PropositionType.Ask && wallet.injectiveAddress === offer.owner)
  )
})
</script>

<template>
  <div class="container pb-32">
    <PartialsCryptopawnHeader />
    <PartialsCryptopawnMenu v-model="activeType" />
    <div class="grid gap-8 grid-cols-2">
      <PartialsCryptopawnOffersList :offerType="PropositionType.Bid" :list-type="activeType" :offers="lendOffers">
        <template v-slot:title>
          <h3 class="font-bold text-2xl">Give a loan</h3>
        </template>
        <template v-slot:tableHeader>
          <tr class="h-14 border-b border-gray-600">
            <th class="text-qwerty-white text-xs normal-case text-left">Lending</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Collateral deposit</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Premium</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Duration</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Expiry date</th>
            <th class="text-qwerty-white text-xs normal-case text-right">Action</th>
          </tr>
        </template>
      </PartialsCryptopawnOffersList>
      <PartialsCryptopawnOffersList :offerType="PropositionType.Ask" :list-type="activeType" :offers="borrowOffers">
        <template v-slot:title>
          <h3 class="font-bold text-2xl">Get a loan</h3>
        </template>
        <template v-slot:tableHeader>
          <tr class="h-14 border-b border-gray-600">
            <th class="text-qwerty-white text-xs normal-case text-left">Borrowing</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Collateral deposit</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Premium</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Duration</th>
            <th class="text-qwerty-white text-xs normal-case text-center">Expiry date</th>
            <th class="text-qwerty-white text-xs normal-case text-right">Action</th>
          </tr>
        </template>
      </PartialsCryptopawnOffersList>
    </div>
    <PartialsCryptopawnCreateOffer />
    <PartialsCryptopawnNotifyModal />
    <PartialsCryptopawnOfferDetails />
  </div>
</template>
