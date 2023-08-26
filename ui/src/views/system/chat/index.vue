<template>
  <div class="app-container">
    <el-row>{{connected?'连接成功':'连接关闭'}}</el-row>
    <el-row>
      <el-col>
        <el-input v-model="sendMsg" />
      </el-col>
      <el-col>
        <el-button type="primary" @click="subSendMsgHandler">提交</el-button>
      </el-col>
    </el-row>
    <el-row>
      <TransitionGroup name="list">
        <el-col v-for="(item,index) in msgArr" :key="index">
          <div :style="userName!=='admin'?(item.name===userName?'text-align:left':'text-align:right'):(index%2===0?'text-align:left':'text-align:right')">
            <p>user:{{ item.name }}</p>
            <p>msg:{{ item.msg }}</p>
          </div>
        </el-col>
      </TransitionGroup>
    </el-row>
  </div>
</template>
<script setup>
import {ElMessage} from "element-plus";
import {computed, nextTick} from "vue";
import useUserStore from "@/store/modules/user";
const userStore = useUserStore()

const token = computed(()=>userStore.token)
const userName = computed(()=>userStore.name)

const ws = new WebSocket(`ws://localhost:8090/chat`);

const sendMsg = ref('')
const msgArr = ref([])
const connected = ref(false)

function wsSendMsg(msg) {
  ws.send(JSON.stringify(
{
        token:token.value,
        name:userName.value,
        msg
      }
  ))
  msgArr.value.push({
    name:userName.value,
    msg
  })

}

ws.onopen = function() {
  connected.value = true
};

ws.onmessage = function(msg) {
  const res = JSON.parse(msg.data);
  if (res.code === 0) {
    msgArr.value.push(res.data)
  }else{
    ElMessage({ message: res.msg, type: 'error' })
  }
};

ws.onclose = function() {
  connected.value = false
};

const subSendMsgHandler = ()=>{
  if(!sendMsg){
    ElMessage({ message: '发送内容不能为空', type: 'error' })
    return
  }
  wsSendMsg(sendMsg.value);
  nextTick(()=>{
    sendMsg.value = ''
  })
}
</script>

<style scoped lang="scss">
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
/* 确保将离开的元素从布局流中删除
  以便能够正确地计算移动的动画。 */
.list-leave-active {
  position: absolute;
}
</style>