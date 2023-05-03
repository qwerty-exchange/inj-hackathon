<!-- <template>
  <label
    :class="labelClass"
    class="
      group
      relative
      rounded-lg
      py-5
      px-4
      items-center
      text-sm
      font-medium
      focus:outline-none
      sm:flex-1
      text-qwerty-white
      cursor-pointer
      flex
      border border-qwerty-shade3
    "
  >
    <span class="mb-2 block lowercase">{{ label }}</span>
    <input
      :name="name"
      type="radio"
      :checked="checked"
      :value="value"
      @input="handleChange"
    />
    <span class="pointer-events-none absolute -inset-px rounded-md" aria-hidden="true"></span>
  </label>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  model: {
    prop: 'value',
    event: 'input'
  },

  props: {
    name: {
      required: true,
      type: String
    },

    type: {
      default: 'block',
      type: String
    },

    label: {
      required: true,
      default: '',
      type: String
    },

    input: {
      type: [String, Number, Boolean],
      default: -1
    },
    value: {
      type: [String, Number, Boolean],
      required: true
    },

    disabled: {
      type: Boolean,
      default: false
    },

    dataCy: {
      type: String,
      default: 'unknown-id'
    }
  },

  data() {
    return {
      checked: false
    }
  },

  computed: {
    uid(): string {
      return window.crypto.getRandomValues(new Uint32Array(1))[0].toString()
    },

    labelClass() {
      const classes = []
      if (this.type === 'block') {
        classes.push('justify-center', 'uppercase', 'flex-col', 'mb-4')
      } else {
        classes.push('justify-between')
      }

      return classes
    }
  },

  watch: {
    input: {
      handler(input) {
        this.checked = input === this.value
      },
      immediate: true
    },
    value: {
      handler(val) {
        this.checked = val === this.input
      },
      immediate: true
    }
  },

  methods: {
    handleChange() {
      this.$emit('input', this.value)
    }
  }
})
</script> -->
