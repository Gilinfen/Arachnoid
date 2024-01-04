import LogViewer from '../components/log'
import { useCallback } from 'react'
import { Button, Form, Input, message } from 'antd'
import { tyInvoke } from '../invoke'
import './App.css'

type FieldType = {
  uuid?: string
  signature?: string
}
const uuid = '9BC0C020-744A-56E6-95D2-6C2402D1EDCC'
const signature =
  'H5d4veI6yfBdgPYC7yFZps32zkw89qmIH9l2Ed+FcHJU66GpdIk3v1CffgBTQUqACedUl4HBnJKTP3I5B0eSFFs7ymGV6wMyfhFzkSBRLqs/F1svtAM9F9Vzbjct+jCT0LNmLEG3pR0t5Px/LM3ReOX1MQbvNHkZNzrZ9/Sfe4tfnqwCMZFnSDlGC3bUoLKUWH7DHlz5mKomx+e+moPq/n7yNKar1bqJKb57tA8ekvd5eL8joWpm3t+BDnLeyKWOvX6rPa1Cmn/62Q3clKBWRnb/fhrkh1riZtjMCjEvKCVU+dkyWgRligk5VC/OocyayXrYkrk6c5CKKed3q2ubLg'

function App() {
  const onFinish = useCallback((values: FieldType) => {
    tyInvoke('use_verify_signature', {
      data: values.uuid,
      signature: values.signature,
    })
      .then((res: any) => {
        if (res) {
          message.success('验证成功')
        } else {
          message.error('验证失败')
        }
      })
      .catch((err) => {
        message.error(err)
      })
  }, [])

  return (
    <div className="container">
      <h1
        style={{
          marginBottom: 16,
        }}
      >
        激活应用
      </h1>
      <Form
        name="basic"
        onFinish={onFinish}
        autoComplete="off"
        labelCol={{
          span: 4,
        }}
        initialValues={{
          uuid,
          signature,
        }}
      >
        <Form.Item<FieldType>
          label="用户标识"
          name="uuid"
          rules={[{ required: true, message: '请输入用户标识!' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="激活码"
          name="signature"
          rules={[{ required: true, message: '请输入激活码!' }]}
        >
          <Input.Password />
        </Form.Item>

        <Form.Item>
          <Button type="primary" htmlType="submit" block>
            激活
          </Button>
        </Form.Item>
      </Form>
      <LogViewer />
    </div>
  )
}

export default App
