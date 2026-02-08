export async function requestOtp(phone: string) {
  const res = await fetch('http://localhost:8080/otp/request', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ phone }),
  })
  return res.json()
}

export async function verifyOtp(phone: string, code: string) {
  const res = await fetch('http://localhost:8080/otp/verify', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ phone, code}),
  })
  return res.json()
}
