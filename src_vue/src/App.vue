<template>
  <v-app>
    <v-app-bar
      app
      color="indigo"
      dark
    >
      <v-toolbar-title class="headline">
        <span>Webスピログラフ</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn
        link
        href="https://github.com/pirosuke/spirograph-rs"
        target="_blank">
        ソースはこちら
      </v-btn>
    </v-app-bar>

    <v-main>
      <v-container fluid class="fill-height">
        <v-layout row>
          <v-flex grow pa-1>
            <router-view/>
          </v-flex>
        </v-layout>
      </v-container>
    </v-main>
    <v-footer
      color="indigo"
      app
    >
      <span class="white--text">&copy; 2021</span>
    </v-footer>
  </v-app>
</template>

<script>
import { mapGetters, mapState } from 'vuex'

export default {
  name: 'App',
  components: {
  },
  data () {
    return {
    }
  },
  computed: {
    ...mapState({
    })
  },

  mounted() {
  },

  created () {
  },

  destroyed () {
  },

  methods: {
  },
}
</script>
