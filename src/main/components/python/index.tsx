import { Button, Form, Input, message } from 'antd'
import { tyInvoke } from '../../../invoke'
import { open } from '@tauri-apps/api/dialog'

type FieldType = {
  [key: string]: string
}

type PythonCmdType = {
  title: string
  cmdType: '-pm' | '-pmxls'
  cmdList: {
    type: 'text' | 'file'
    name: string
  }[]
}

const OpFileInput = ({ value, onChange }: any) => {
  return (
    <>
      <Button
        onClick={async () => {
          const value = await open({
            directory: true,
          })
          onChange(value)
        }}
      >
        选择文件
      </Button>
      {value}
    </>
  )
}

export default function PythonCmd({ cmdType, cmdList, title }: PythonCmdType) {
  const onFinish = async (values: FieldType) => {
    tyInvoke('execute_python_script', {
      cmds: [cmdType, ...Object.values(values)],
    })
      .then((_) => {
        message.success('处理成功')
      })
      .catch((err) => {
        message.error(err)
      })
  }

  return (
    <div>
      <h2>{title}</h2>
      <Form name="basic" onFinish={onFinish} autoComplete="off">
        {cmdList.map((item) => (
          <Form.Item<FieldType>
            label={item.name}
            name={item.name}
            rules={[{ required: true, message: 'Please input your username!' }]}
          >
            {item.type === 'file' ? <OpFileInput /> : <Input />}
          </Form.Item>
        ))}
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    </div>
  )
}
