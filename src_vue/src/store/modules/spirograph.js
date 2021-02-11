let spiroAPI

export default {
    namespaced: true,
    state: {
        point: {
            x: 0,
            y: 0,
        },
    },
    mutations: {
        setPoint(state, payload) {
            state.point = payload.point
        },
    },
    actions: {
        async loadWasm(context) {
            const wasm = import("../../../../pkg")
            spiroAPI = await wasm
        },
        async calcNewPoint(context, payload) {
            if (spiroAPI == undefined) {
                await context.dispatch('loadWasm')
            }
            const newPoint = await spiroAPI.calc_point(JSON.stringify(payload.settings), payload.angle)
            context.commit("setPoint", {
                point: JSON.parse(newPoint),
            })
        },
    },
}