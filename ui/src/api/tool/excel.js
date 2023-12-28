import request from '@/utils/request'

// 创建excel
export function addExcel(data) {
  return request({
    url: '/tool/excel/add',
    method: 'post',
    data
  })
}

// 修改excel
export function editExcel(data) {
  return request({
    url: '/tool/excel/edit',
    method: 'post',
    data
  })
}

// 获取excel列表
export function getExcelList() {
  return request({
    url: '/tool/excel/list',
    method: 'get'
  })
}

// 获取excel列表
export function getExcelById(excelIdd) {
  return request({
    url: '/tool/excel/'+excelIdd,
    method: 'get'
  })
}