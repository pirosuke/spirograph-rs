import Vue from "vue";
import Vuex from "vuex";
import spirograph from './modules/spirograph'

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    spirograph,
  }
});
