<template>
  <div
    id="navigation-panel"
    class="w-72 border-r border-indigo-800 bg-goose-100"
  >
    <div
      class="location-bar flex flex-row bg-indigo-800 items-center pl-4 pr-4 h-12 bz-right-15px mr-negative-15px"
    >
      <div
        v-if="namespace.length > 0"
        class="nav-translation-key nav-back text-white"
      >
        <a href="#" @click.prevent="goToParentNamespace(namespace)">
          <span
            ><b>{{ namespace }}</b></span
          >
        </a>
      </div>
    </div>
    <ul class="nav-translation-key-list">
      <template v-for="key in filteredTranslationsKeys" :key="key">
        <NavigationKey
          :canonicalKey="key"
          :namespace="namespace"
          :forceCanonical="false"
          @namespaceChanged="setNamespace(...arguments)"
        />
      </template>
    </ul>
  </div>
</template>

<script>
import NavigationKey from "./NavigationKey.vue";
import _ from "lodash";

export default {
  name: "navigation-bar",

  props: {
    translationKeys: Array,
    namespace: String,
  },

  computed: {
    filteredTranslationsKeys() {
      return this.currentNamespaceTranslationKeys;
    },

    currentNamespaceTranslationKeys() {
      let currentKeys = _.filter(this.translationKeys, (key) => {
        return key.startsWith(this.namespace || "");
      });

      let truncated = _.map(currentKeys, (key) => {
        let namespaceDepth = 0;
        if (this.namespace.length > 0) {
          namespaceDepth = (this.namespace.match(/\./g) || []).length + 1;
        }
        let keyWithoutNamespace = key
          .split(".")
          .slice(namespaceDepth)
          .join(".");
        let crumbs = keyWithoutNamespace.split(".");
        let initialDot = this.namespace.length > 0 ? "." : "";

        if (crumbs.length > 1) {
          return this.namespace + initialDot + crumbs[0] + "..";
        } else {
          return this.namespace + initialDot + keyWithoutNamespace;
        }
      });

      return _.uniq(truncated).sort();
    },
  },

  methods: {
    setNamespace(namespace) {
      this.$emit("namespaceChanged", namespace);
    },
    goToParentNamespace(namespace) {
      this.$emit(
        "namespaceChanged",
        namespace.split(".").slice(0, -1).join(".")
      );
    },
  },

  components: {
    NavigationKey,
  },
};
</script>
