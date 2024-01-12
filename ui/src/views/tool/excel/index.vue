<script setup>
  import { ElMessage } from 'element-plus'
  import eSheet from 'e-sheet'
  import 'e-sheet/dist/css/index.css'
  import {onMounted,ref,reactive} from "vue";
  import useUserStore from '@/store/modules/user'
  import {addExcel, getExcelList, getExcelById,editExcel} from '@/api/tool/excel'

  const excelDom = ref(null)
  const dialogVisible = ref(false)
  const excelName = ref('')
  const excelList = ref([])
  const curExcel = reactive({
    excelId:'',
    excelData:null,
  })


  function devTip() {
    ElMessage({
      type:'warning',
      message:'功能开发中'
    })
  }

  function handleClose() {
    dialogVisible.value = false
  }

  let excelPkg = null

  onMounted(()=>{
    excelPkg = new eSheet(excelDom.value,{
      width:excelDom.value.clientWidth,
      height:700,
      init:false
    })
    const userStore = useUserStore()
    // console.log('store.state.userName',userStore.name)
    excelPkg.setUserName(userStore.name)
    freshList().then(data=>{
      curExcel.excelId = data[0].excelId
      getExcelDataById(curExcel.excelId).then(()=>{
        excelPkg.connectWebSocket('ws://localhost:8090/tool/excel/connected')
      })
    })
    excelPkg.stepCallbackHandle(()=>{
      // console.log('excelPkg.eSheetWorkBook',excelPkg.eSheetWorkBook)
      setTimeout(()=>{
        editExcel({
          excelId:curExcel.excelId,

          excelData:JSON.stringify(excelPkg.eSheetWorkBook.map(item=>{
            return {
              id:item.id,
              label:item.label,
              sheet:item.sheet
            }
          }))
        })
      },2000)
    })
  })

  function openAddDialog() {
    dialogVisible.value = true
  }
  
  function creatNewExcel() {
    addExcel({
      excelName:excelName.value
    }).then(res=>{
      handleClose()
      curExcel.excelId = res.data
      freshList()
      excelPkg.drawExcel()
    })
  }

  function getExcelDataById(excelId) {
    return new Promise((resolve, reject) => {
      getExcelById(excelId).then(res=>{
        curExcel.excelId = excelId
        // const data = JSON.parse(res.data.excelData)
        // console.log('curExcel.excelData',data[0].sheet)
        // try {
        // console.log('JSON.parse(res.data.excelData)',JSON.parse(res.data.excelData))
        res.data.excelData && excelPkg.drawExcel(JSON.parse(res.data.excelData))
        console.log('excelPkg.sheetWidth',excelPkg.sheetWidth)
        // }catch (e) {
        //   console.log(' excelPkg.getCurrentSheet()', excelPkg.getCurrentSheet())
        // }
        resolve()
      }).catch(e=>{
        reject(e)
      })
    })

  }
  
  function freshList() {
    return new Promise((resolve, reject) => {
      getExcelList().then(res=>{
        excelList.value = res.data
        resolve(res.data)
      }).catch(e=>{
        reject(e)
      })
    })

  }

</script>

<template>
  <div class="excel-layout">
    <div class="left-side">
      <div>
        <el-button @click="openAddDialog" type="primary">创新新文档</el-button>
      </div>
      <ul class="excel-list-layout">
        <li :class="curExcel.excelId === item.excelId?'selected':''" @click="devTip" v-for="item in excelList" :key="item.excelId">{{item.excelName}}</li>
      </ul>
    </div>
    <div class="right-content">
      <div ref="excelDom"></div>
    </div>
    <el-dialog
        v-model="dialogVisible"
        title="请输入文档名称"
        width="30%"
        :before-close="handleClose"
    >
      <el-input v-model="excelName"></el-input>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="handleClose">取消</el-button>
          <el-button type="primary" @click="creatNewExcel">
            创建
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped lang="scss">
.excel-layout{
  display: flex;
  padding: 0 10px;
  .left-side{
    width: 300px;
    .excel-list-layout{
      list-style: none;
      padding-left: 0;
      .selected{
        background: #1c84c6;
        color: #FFFFFF;
      }
    }
  }
  .right-content{
    flex: 1;
  }
}
</style>