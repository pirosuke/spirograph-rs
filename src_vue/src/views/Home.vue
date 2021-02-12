<template>
  <v-container>
    <v-row>
      <v-col>
        <v-container>
          <v-row>
            <v-col>
              <canvas class="spirograph" ref="canvas" width="500" height="500"></canvas>
            </v-col>
          </v-row>
          <v-row>
            <v-col>
              <v-progress-linear
                color="light-blue"
                height="10"
                :value="progress"
                striped
              ></v-progress-linear>
            </v-col>
          </v-row>
        </v-container>
      </v-col>
      <v-col>
        <v-card>
          <v-card-text>
            <v-container>
              <v-row>
                <v-col>
                  <v-slider
                    label="定円の半径"
                    v-model="outer_radius"
                    min="1"
                    max="200"
                    step="1"
                    thumb-label="always"
                  >
                  </v-slider>
                </v-col>
              </v-row>
              <v-row>
                <v-col>
                  <v-slider
                    label="動円の半径"
                    v-model="inner_radius"
                    min="1"
                    max="100"
                    step="1"
                    thumb-label="always"
                  >
                  </v-slider>
                </v-col>
              </v-row>
              <v-row>
                <v-col>
                  <v-slider
                    label="描画点の半径"
                    v-model="draw_radius"
                    min="1"
                    max="100"
                    step="1"
                    thumb-label="always"
                  >
                  </v-slider>
                </v-col>
              </v-row>
              <v-row>
                <v-col>
                  <v-slider
                    label="回転数"
                    v-model="rotation_times"
                    min="10"
                    max="100"
                    step="10"
                    thumb-label="always"
                  >
                  </v-slider>
                </v-col>
              </v-row>
              <v-row>
                <v-col>
                  <v-color-picker
                    v-model="color"
                    hide-inputs>
                  </v-color-picker>
                </v-col>
              </v-row>
              <!-- v-row>
                <v-col>
                  <v-text-field
                    label="角度の間隔"
                    v-model="angle_update_coef"
                    type="number"
                  >
                  </v-text-field>
                </v-col>
              </v-row -->
            </v-container>
          </v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn color="blue darken-1" @click="draw">描画実行</v-btn>
            <v-btn color="blue darken-1" @click="clearCanvas">キャンバスをクリア</v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
  import { mapGetters, mapState } from 'vuex'

  export default {
    data () {
      return {
        outer_radius: 130,
        inner_radius: 34,
        draw_radius: 47,
        angle_update_coef: 0.1,
        rotation_times: 20,
        hexColor: "#000000",
        angle: 0,
      }
    },

    watch: {
    },

    async mounted() {
    },
    computed: {
        ...mapState({
        }),
        color: {
          get() {
            return this.hexColor
          },
          set(v) {
            this.hexColor = v
          },
        },
        maxAngle() {
          return 2 * Math.PI * this.rotation_times
        },
        progress() {
          return this.angle / this.maxAngle * 100
        },
    },
    methods: {
      async draw() {
        const canvas = this.$refs.canvas
        const ctx = canvas.getContext("2d")

        //this.ctx.lineWidth = 1
        ctx.strokeStyle = this.color
        ctx.beginPath()

        this.angle = 0
        while (this.angle < this.maxAngle) {
          await this.$store.dispatch('spirograph/calcNewPoint', {
            settings: {
              outer_radius: this.outer_radius,
              inner_radius: this.inner_radius,
              draw_radius: this.draw_radius,
              area_width: canvas.width,
              area_height: canvas.height,
            },
            angle: this.angle,
          })
          const point = this.$store.state.spirograph.point
          ctx.lineTo(point.x, point.y)
          ctx.stroke()
          this.angle += this.angle_update_coef
          await this.sleep()
        }
      },

      clearCanvas() {
        const canvas = this.$refs.canvas
        const ctx = canvas.getContext("2d")
        ctx.clearRect(0, 0, canvas.width, canvas.height)
      },

      sleep() {
        return new Promise(requestAnimationFrame)
      },
    }
  }
</script>

<style scoped>
</style>