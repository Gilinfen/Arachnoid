import { Button, Form, message } from 'antd'
import { tyInvoke } from '../../../invoke'
import { open } from '@tauri-apps/api/dialog'

type FieldType = {
  [key: string]: string
}

type CmdType = '-pm' | '-pmxls'

type PythonCmdType = {
  title: string
  cmdType: CmdType
  cmdList: string[]
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
            label={item}
            name={item}
            rules={[{ required: true, message: 'Please input your username!' }]}
          >
            <OpFileInput />
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
