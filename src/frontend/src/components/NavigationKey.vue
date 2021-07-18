<template>
  <li class="nav-translation-key">
    <a
      class="flex items-center text-sm px-2 h-9"
      :href="`#${canonicalKey}`"
      @click.prevent="setNamespace(canonicalKey)"
    >
      <template v-if="isFolder">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 fill-current mr-2 flex-none"
          viewBox="0 0 20 20"
        >
          <path
            d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z"
          />
        </svg>
      </template>
      <template v-else>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 fill-current mr-2 flex-none"
          viewBox="0 0 20 20"
        >
          <path d="M5 10a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1z" />
        </svg>
      </template>

      <span class="flex-grow">{{ label }}</span>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5 flex-none"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
        />
      </svg>
    </a>
  </li>
</template>

<script>
import _ from "lodash";

export default {
  name: "navigation-key",

  props: {
    canonicalKey: String,
    namespace: String,
    forceCanonical: Boolean,
  },

  computed: {
    label() {
      if (this.forceCanonical) {
        return this.canonicalKey;
      } else {
        let key = this.canonicalKey;

        // Remove .. at the end of folders key
        if (this.isFolder) {
          key = key.slice(0, -2);
        }

        // Remove namespace from canonical (including dot)
        if (this.namespace.length > 0) {
          key = key.substring(this.namespace.length + 1);
        }

        return key;
      }
    },

    isFolder() {
      return this.canonicalKey.slice(-2) == "..";
    },
  },

  methods: {
    setNamespace(namespace) {
      this.$emit("namespaceChanged", namespace.replace(/\.\.$/, ""));
    },
  },
};
</script>