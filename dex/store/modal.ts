import { defineStore } from 'pinia'
import { Modal, ModalState } from '@/types'

const modalValues = Object.values(Modal)
const modalExists = (modal: Modal) => modalValues.includes(modal)

export type ModalMap = Record<Modal, boolean>
export type ModalStoreState = {
  modals: ModalMap
  persistModal?: string
  data?: any
  preventScroll: boolean
}

const generateModalsMap = () =>
  modalValues.reduce((previous: ModalState, current: Modal) => {
    return { ...previous, [current]: false }
  }, {} as ModalState)

const initialStateFactory = (): ModalStoreState => ({
  modals: generateModalsMap() as ModalMap,
  persistModal: undefined,
  preventScroll: false,
  data: {}
})

export const useModalStore = defineStore('modal', {
  state: (): ModalStoreState => initialStateFactory(),
  actions: {
    openPersistedModalIfExist() {
      const modalStore = useModalStore()

      if (!modalStore.persistModal) {
        return
      }

      modalStore.openModal({ type: modalStore.persistModal as Modal })

      modalStore.$patch({
        persistModal: undefined
      })
    },

    closeModal(modal: Modal) {
      const modalStore = useModalStore()

      if (modalExists(modal) && modalStore.modals[modal]) {
        if (modalStore.preventScroll) {
          document.body.style.overflowY = 'auto'
        }

        modalStore.$patch({
          modals: { ...modalStore.modals, [modal]: false }
        })
      }
    },

    openModal(modal: { type: Modal; preventScroll?: boolean; data?: any }) {
      if (modalExists(modal.type)) {
        const modalStore = useModalStore()

        if (modal.preventScroll) {
          document.body.style.overflowY = 'hidden'
        }

        modalStore.$patch({
          modals: { ...modalStore.modals, [modal.type]: true },
          preventScroll: modal.preventScroll,
          data: {
            ...modalStore.data,
            [modal.type]: modal.data
          }
        })
      }
    },

    reset() {
      const modalStore = useModalStore()

      modalStore.$patch({
        ...initialStateFactory()
      })
    }
  }
})
