<template>
  <div class="translation-key px-2 h-8 text-sm text-gray-600 flex items-center">
    <span class="w-16" />
    <span
      v-for="(crumb, index) in crumbs"
      :class="`translation-crumb flex items-center ${
        index == crumbs.length - 1 ? 'flex-grow' : ''
      }`"
      :key="index"
      ><svg
        v-if="index > 0"
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
          clip-rule="evenodd"
        /></svg
      ><span class="text-0">.</span>{{ crumb }}</span
    >
    <button
      class="translation-key-delete"
      @click="deleteTranslation"
      tabindex="-1"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
          clip-rule="evenodd"
        />
      </svg>
    </button>
  </div>
</template>

<script>
export default {
  name: "translation-key",

  props: {
    translationKey: String,
  },

  computed: {
    crumbs() {
      return this.translationKey.split(".");
    },
  },

  methods: {
    deleteTranslation() {
      let confirmation = window.confirm(
        `You will destroy ${this.translationKey}; are you sure?`
      );

      if (confirmation) {
        this.$root.store
          .callApi(`/api/v1/translations/${this.translationKey}`, "DELETE")
          .then((response) => {
            if (response.ok) {
              return response.json();
            } else {
              window.alert("An error occured while deleting the translation.");
            }
          })
          .then((data) => {
            this.$root.store.fetchTranslations();
          });
      }
    },
  },
};
</script>
