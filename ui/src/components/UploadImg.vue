<template>
  <div class="upload-img-layout">
    <div class="img-layout" v-for="(it,i) in value" :key="i">
      <img class="img" :src="it">
      <div class="remove" @click="removeImg(i)">+</div>
    </div>
    <div class="file-upload" @click="startUpload">+</div>
    <input type="file" ref="fileUpload" style="display:none" :multiple="multiple" @change="uploadFile">
  </div>
</template>

<script>
export default {
  props:{
    value:{
      type:Array,
      default:()=>[]
    },
    limit:{
      type:Number,
      default:0
    },
    multiple:{
      type:Boolean,
      default:false
    }
  },
  data(){
    return {
      uploadaction:
        this.constant.loadurl.uploadThumbnail +
        "?token=" +
        localStorage.getItem("token"), //上传缩略图路径
      dataImgs:[]
    }
  },
  methods:{
    removeImg(i){
      this.dataImgs.splice(i,1)
      this.$emit('input',[...this.dataImgs])
    },
    startUpload(){
      this.$refs['fileUpload'].value = ''
      this.$refs['fileUpload'].click()
      this.dataImgs = this.value
    },
    uploadFile(e){
      // console.log('组件内部的上传图片',this.dataImgs)
      if(this.value.length >= this.limit && this.limit === 0){
        // 没有限制
      }
      if(this.value.length >= this.limit){
        this.$message.error('超出上传限制数量，上传失败')
        return
      }
      const formData = new FormData();
      for(let i=0;i<e.target.files.length;i++){
        formData.append('file',e.target.files[i])
      }
      // console.log('formData',formData)
      const vm = this;
      vm.$http
        .post(vm.uploadaction, formData, {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        }).then(res=>{
          if(res.data.code === 0){
            if(e.target.files.length>1){
              const list = res.data.data.src.split(',')
              for(let i=0;i<list.length;i++){
                this.dataImgs.push(list[i])
              }
            }else{
              this.dataImgs.push(res.data.data.src)
            }
            this.$emit('input',[...this.dataImgs])
          }
        })
    }
  }
}
</script>

<style lang="scss" scoped>
.upload-img-layout{
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  .img-layout{
    position: relative;
    width: 100px;
    height: 100px;
    margin-right: 10px;
    .img{
      width: 100px;
      height: 100px;
    }
    .remove{
      width: 20px;
      height: 20px;
      border-radius: 50%;
      position: absolute;
      top: 0px;
      right: 0;
      font-size: 20px;
      transform: rotate(45deg);
      background: #fff;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
    }
  }
  .file-upload{
    width: 100px;
    height: 100px;
    color: #c0c0c0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 60px;
    border:1px solid #000;
    border-radius: 6px;
    cursor: pointer;
  }
}

</style>