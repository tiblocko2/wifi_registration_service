import { useEffect, useState } from "react";
import { requestOtp, verifyOtp } from "../api/otp";

export function AuthPage() {
  const [step, setStep] = useState<"phone" | "code" | "success">("phone");
  const [phone, setPhone] = useState("");
  const [code, setCode] = useState("");
  const [error, setError] = useState("");
  const [timer, setTimer] = useState(0);

  // капча
  const [captcha] = useState(() =>
    Math.floor(1000 + Math.random() * 9000).toString()
  );
  const [captchaInput, setCaptchaInput] = useState("");

  /* ================= TIMER ================= */

  useEffect(() => {
    if (timer <= 0) return;

    const interval = setInterval(() => {
      setTimer((prev) => {
        if (prev <= 1) {
          localStorage.removeItem("otp_timer_expires");
          clearInterval(interval);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [timer]);

  /* ================= REQUEST ================= */

  async function handleRequestOtp() {
    setError("");

    if (captchaInput !== captcha) {
      setError("Неверная капча");
      return;
    }

    try {
      await requestOtp("7" + phone);
      setStep("code");

      const expiresAt = Date.now() + 240_000;
      localStorage.setItem("otp_timer_expires", expiresAt.toString());
      setTimer(240);
    } catch {
      setError("Ошибка отправки кода");
    }
  }

  async function handleVerifyOtp() {
    setError("");
    const res = await verifyOtp("7" + phone, code);

    if (res.status === "verified") {
      setStep("success");
    } else if (res.status === "expired") {
      setError("Код истёк");
    } else {
      setError("Неверный код");
    }
  }

  /* ================= UI ================= */

  return (
    <div style={styles.page}>
      <div style={styles.card}>
        <h1 style={styles.logo}>Dr.Mun</h1>

        {step === "phone" && (
          <>
            <h2>Введите номер</h2>

            <div style={styles.phoneRow}>
              <span style={styles.phonePrefix}>+7</span>
              <input
                style={styles.phoneInput}
                value={phone}
                onChange={(e) =>
                  setPhone(e.target.value.replace(/\D/g, "").slice(0, 10))
                }
                placeholder="9---------"
              />
            </div>

            <div style={styles.captcha}>
              <span style={styles.captchaCode}>{captcha}</span>
              <input
                style={styles.input}
                value={captchaInput}
                onChange={(e) =>
                  setCaptchaInput(
                    e.target.value.replace(/\D/g, "").slice(0, 4)
                  )
                }
                placeholder="Введите капчу"
              />
            </div>

            <button
              style={styles.button}
              disabled={phone.length !== 10 || timer > 0}
              onClick={handleRequestOtp}
            >
              Получить код
            </button>

            {error && <p style={styles.error}>{error}</p>}
          </>
        )}

        {step === "code" && (
          <>
            <h2>Введите код</h2>
            <input
              style={styles.input}
              value={code}
              onChange={(e) =>
                setCode(e.target.value.replace(/\D/g, "").slice(0, 6))
              }
              placeholder="6 цифр"
            />

            <button
              style={styles.button}
              disabled={code.length !== 6}
              onClick={handleVerifyOtp}
            >
              Подтвердить
            </button>

            {timer > 0 ? (
              <p>Отправить снова через {timer} сек</p>
            ) : (
              <button style={styles.linkBtn} onClick={handleRequestOtp}>
                Отправить снова
              </button>
            )}

            {error && <p style={styles.error}>{error}</p>}
          </>
        )}

        {step === "success" && (
          <h2 style={{ color: "#00ffcc" }}>✅ Подтверждено</h2>
        )}
      </div>
    </div>
  );
}

/* ================= STYLES ================= */

const styles = {
  page: {
    minHeight: "100vh",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    backgroundImage: `url('/back.jpg')`,
    backgroundSize: "cover",
    padding: "16px",
  },

  card: {
    width: "100%",
    maxWidth: "360px",
    padding: "20px",
    borderRadius: "16px",
    background: "rgba(10, 20, 40, 0.65)",
    border: "1px solid rgba(0, 180, 255, 0.6)",
    display: "flex",
    flexDirection: "column" as const,
    gap: "12px",
    textAlign: "center" as const,
    color: "#e6f1ff",
  },

  logo: {
    fontSize: "28px",
    marginBottom: "8px",
    color: "transparent",
    WebkitTextStroke: "1px rgba(0,180,255,0.9)",
    textShadow: "0 0 6px rgba(0,180,255,0.7)",
  },

  phoneRow: {
    display: "flex",
    alignItems: "center",
    gap: "10px",
  },

  phonePrefix: {
    fontSize: "20px",
    fontWeight: 600,
    color: "#9ecbff",
    lineHeight: 1,
    marginTop: "-15px",
  },

  phoneInput: {
    flex: 1,
    padding: "12px",
    borderRadius: "10px",
    border: "1px solid #1e90ff",
    background: "rgba(0,0,0,0.35)",
    color: "#fff",
    fontSize: "16px",
    outline: "none",
  },

  input: {
    padding: "12px",
    borderRadius: "10px",
    border: "1px solid #1e90ff",
    background: "rgba(0,0,0,0.35)",
    color: "#fff",
    fontSize: "16px",
    outline: "none",
    boxSizing: "border-box" as "border-box",
  },

  button: {
    padding: "12px",
    borderRadius: "10px",
    border: "none",
    background: "#1e90ff",
    color: "#fff",
    cursor: "pointer",
    fontSize: "16px",
  },

  linkBtn: {
    background: "none",
    border: "none",
    color: "#1e90ff",
    cursor: "pointer",
    fontSize: "14px",
  },

  captcha: {
    display: "flex",
    flexDirection: "column" as const,
    gap: "6px",
  },

  captchaCode: {
    letterSpacing: "4px",
    fontSize: "18px",
    color: "#00ccff",
  },

  error: {
    color: "#ff6b6b",
    fontSize: "14px",
  },
};
