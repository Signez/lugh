<template>
  <div
    :class="[`translation flex`, { edited: modified }]"
    :t-id="translation.id"
  >
    <div
      class="translation-locale-label w-16 px-2 py-2 text-right font-mono text-sm font-bold"
    >
      {{ translation.locale }}
    </div>
    <div class="translation-editor relative bg-white flex flex-col flex-grow">
      <textarea
        :class="`flex-grow px-3 py-2 pb-6 text-sm outline-none transition-colors ${
          modified ? 'bg-yellow-50' : ''
        }`"
        :lang="translation.locale"
        v-model="localContent"
        ref="translationTextarea"
        @keyup.esc="revertOrBlur"
        @keyup.ctrl.enter="save"
        @keyup.meta.enter="save"
      ></textarea>
      <transition name="hint-animation">
        <span
          class="keyboard-hint font-light text-xs absolute bottom-0 pb-2 pl-3 text-yellow-800"
          v-if="modified"
        >
          <span class="inline-block -ml-9 mr-6" v-if="modified">
            <span
              class="block w-3 h-3 bg-yellow-600 rounded-full animate-ping"
            />
          </span>
          <b class="font-medium">Ctrl-Enter</b> saves your changes.
          <b class="font-medium">Escape</b> discards them.
        </span>
      </transition>
      <transition name="hint-animation">
        <span
          class="just-saved flex items-center font-light text-xs absolute bottom-0 pb-2 pl-3 text-green-400"
          v-if="justSaved"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 mr-1"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
              clip-rule="evenodd"
            />
          </svg>
          <span> Saved successfully! </span>
        </span>
      </transition>
    </div>
  </div>
</template>

<script>
export default {
  name: "translation-locale",

  beforeUpdate() {
    this.$nextTick(() => {
      if (this.contentForKey !== this.translationKey) {
        this.localContent = this.translation.content;
        this.contentForKey = this.translationKey;
      }
    });
  },

  data() {
    return {
      localContent: this.translation.content,
      contentForKey: this.translationKey,
      justSaved: false,
    };
  },

  props: {
    translation: Object,
    translationKey: String,
  },

  computed: {
    modified() {
      return (
        String(this.localContent).valueOf() !==
        String(this.translation.content).valueOf()
      );
    },
  },

  methods: {
    revertOrBlur() {
      if (this.modified) {
        this.revert();
      } else {
        this.$refs.translationTextarea.blur();
      }
    },

    revert() {
      // Save cursor position and selection before revert
      let start = this.$refs.translationTextarea.selectionStart;
      let end = this.$refs.translationTextarea.selectionEnd;

      // Actual revert
      this.localContent = this.translation.content;

      // Restore cursor position and selection after revert
      this.$nextTick(() => {
        this.$refs.translationTextarea.selectionStart = start;
        this.$refs.translationTextarea.selectionEnd = end;
      });
    },

    save() {
      // Save cursor position and selection before save
      let start = this.$refs.translationTextarea.selectionStart;
      let end = this.$refs.translationTextarea.selectionEnd;

      this.$root.$props.store
        .callApi("/api/v1/translations", "POST", {
          key: this.translationKey,
          locale: this.translation.locale,
          content: this.localContent,
        })
        .then((response) => {
          if (response.ok) {
            return response.json();
          } else {
            window.alert("An error occured while saving the translation.");
          }
        })
        .then((data) => {
          // Apply modification locally for better UI feedback
          this.translation.content = this.localContent;

          // TODO: We should not refresh all locales here
          this.$root.$props.store.fetchTranslations();

          // Restore cursor position and selection after revert
          this.$nextTick(() => {
            this.$refs.translationTextarea.selectionStart = start;
            this.$refs.translationTextarea.selectionEnd = end;
          });

          // Show a little confirmation that disappear after 2 seconds
          this.justSaved = true;
          window.setTimeout(() => {
            this.justSaved = false;
          }, 1000);
        });
    },
  },
};
</script>

<style>
.hint-animation-enter-active {
  animation: 0.2s show-hint;
}
.hint-animation-leave-active {
  animation: 0.2s hide-hint;
}
@keyframes show-hint {
  0% {
    transform: translateY(1.2em) scaleY(0);
    opacity: 0;
  }
  100% {
    transform: none;
    opacity: 1;
  }
}
@keyframes hide-hint {
  100% {
    transform: translateY(-1.2em) scaleY(0);
    opacity: 0;
  }
  0% {
    transform: none;
    opacity: 1;
  }
}
</style>