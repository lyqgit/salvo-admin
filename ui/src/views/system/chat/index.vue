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
      <el-col v-for="(item,index) in msgArr">
        <div :style="index%2===0?'text-align:left':'text-align:right'">
          <p>user:{{ item[0] }}</p>
          <p>msg:{{ item[1] }}</p>
        </div>
      </el-col>
    </el-row>
  </div>
</template>
<script setup>
import {ElMessage} from "element-plus";
import {nextTick} from "vue";

const ws = new WebSocket(`ws://localhost:8090/chat`);

const sendMsg = ref('')
const msgArr = ref([])
const connected = ref(false)

ws.onopen = function() {
  connected.value = true
};

ws.onmessage = function(msg) {
  console.log('msg.data',msg.data);
  msgArr.value.push(msg.data.split(':'))
};

ws.onclose = function() {
  connected.value = false
};

const subSendMsgHandler = ()=>{
  if(!sendMsg){
    ElMessage({ message: '发送内容不能为空', type: 'error' })
    return
  }
  ws.send(sendMsg.value);
  nextTick(()=>{
    sendMsg.value = ''
  })
}
</script>

<style scoped lang="scss">

</style>