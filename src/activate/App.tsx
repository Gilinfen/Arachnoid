import { invoke } from '@tauri-apps/api'
import './App.css'
import Verify from '../components/verify'

function App() {
  return (
    <div className="container">
      <button
        onClick={() => {
          invoke('use_verify_signature', {
            data: '9BC0C020-744A-56E6-95D2-6C2402D1EDCC',
            signature:
              'H5d4veI6yfBdgPYC7yFZps32zkw89qmIH9l2Ed+FcHJU66GpdIk3v1CffgBTQUqACedUl4HBnJKTP3I5B0eSFFs7ymGV6wMyfhFzkSBRLqs/F1svtAM9F9Vzbjct+jCT0LNmLEG3pR0t5Px/LM3ReOX1MQbvNHkZNzrZ9/Sfe4tfnqwCMZFnSDlGC3bUoLKUWH7DHlz5mKomx+e+moPq/n7yNKar1bqJKb57tA8ekvd5eL8joWpm3t+BDnLeyKWOvX6rPa1Cmn/62Q3clKBWRnb/fhrkh1riZtjMCjEvKCVU+dkyWgRligk5VC/OocyayXrYkrk6c5CKKed3q2ubLg',
          }).then(() => {
            console.log('成功')
          })
        }}
      >
        <h1>激活</h1>
      </button>
      <Verify />
    </div>
  )
}

export default App
