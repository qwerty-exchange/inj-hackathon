<script lang="ts" setup>
import { Status, StatusType } from '@injectivelabs/utils'
import { subscribeToNewsletter } from '@/app/services/newsletter'

const { t } = useLang()
const { success, error } = useNotifications()
const { handleSubmit, resetForm } = useForm()

const status = reactive(new Status(StatusType.Idle))

const { value, errors } = useStringField({
  name: 'email',
  rule: 'required|email'
})

const subscribe = handleSubmit((values) => {
  status.setLoading()

  subscribeToNewsletter(values.email)
    .then(() => {
      success({
        title: t('newsletter.subscribeToast')
      })

      resetForm()
    })
    .catch((e: any) => {
      error(e.message.replace('Error', ''))
    })
    .finally(() => {
      status.setIdle()
    })
})
</script>

<template>
  <div class="py-20 bg-qwerty-shade2 text-center">
    <h1 class="text-3xl font-semibold">{{ $t('newsletter.title') }}</h1>

    <div class="max-w-[340px] xs:max-w-[360px] mt-6 mx-auto">
      <div class="bg-gray-750 rounded-lg flex items-center">
        <AppInput
          v-model="value"
          :placeholder="$t('newsletter.emailAddress')"
          transparent-bg
        >
        </AppInput>

        <AppButton
          xl
          class="bg-qwerty-primary text-qwerty-background"
          :disabled="errors.length > 0"
          :status="status"
          @click="subscribe"
        >
          <span class="text-sm">
            {{ $t('newsletter.subscribe') }}
          </span>
        </AppButton>
      </div>
      <div
        v-if="errors.length > 0"
        class="mt-2 text-left text-sm capitalize-phrase"
      >
        <span class="text-qwerty-red">{{ errors[0] }}</span>
      </div>

      <div class="mt-6">
        <i18n-t
          keypath="newsletter.disclaimerMessage"
          tag="p"
          class="text-xs text-qwerty-white"
        >
          <template #termsAndCondition>
            <a
              class="underline hover:text-qwerty-primary"
              href="https://injectivelabs.org/terms-and-conditions"
              target="_blank"
            >
              {{ $t('newsletter.termsAndCondition') }}
            </a>
          </template>

          <template #privacyPolicy>
            <a
              class="underline hover:text-qwerty-primary"
              href="https://injectivelabs.org/privacy-policy"
              target="_blank"
            >
              {{ $t('newsletter.privacyPolicy') }}
            </a>
          </template>

          <template #disclaimer>
            <a
              class="underline hover:text-qwerty-primary"
              href="https://injective.com/disclaimer/"
              target="_blank"
            >
              {{ $t('newsletter.disclaimer') }}
            </a>
          </template>
        </i18n-t>
      </div>
    </div>
  </div>
</template>
