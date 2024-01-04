import PythonCmd from '../../components/python'

export default function Python() {
  return (
    <div>
      <PythonCmd
        title="处理产品目录"
        cmdType="-pm"
        cmdList={[
          {
            name: '产品目录',
            type: 'file',
          },
          {
            name: '目标目录',
            type: 'file',
          },
          {
            name: '删除字符',
            type: 'text',
          },
        ]}
      />
      <PythonCmd
        title="处理成 Excel 文件"
        cmdType="-pmxls"
        cmdList={[
          {
            name: '产品目录',
            type: 'file',
          },
          {
            name: '目标目录',
            type: 'file',
          },
        ]}
      />
    </div>
  )
}
