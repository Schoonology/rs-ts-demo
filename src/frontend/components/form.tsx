import React, { FormEvent, useState } from "react";

export function Form() {
  const [error, setError] = useState("");
  const [message, setMessage] = useState("");
  const [sending, setSending] = useState(false);

  function onSubmit(event: FormEvent) {
    event.preventDefault();

    setSending(true);

    fetch("/posts", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        message,
        timestamp: Date.now(),
      }),
    }).then(
      () => {
        setError("");
        setMessage("");
        setSending(false);
      },
      () => {
        setError("Failed to submit new post.");
        setSending(false);
      }
    );
  }

  return (
    <form onSubmit={onSubmit}>
      <p className="text-red">{error}</p>
      <div className="flex flex-col md:flex-row gap-3">
        <label className="md:flex-1" htmlFor="message">
          <input
            className="border border-cyan-400 p-3 rounded-md w-full"
            type="text"
            name="message"
            placeholder="Message"
            value={message}
            onChange={(event) => setMessage(event.target.value)}
          />
        </label>
        <button
          className="bg-cyan-900 px-6 py-3 rounded-md text-white"
          disabled={sending}
          type="submit"
        >
          Submit
        </button>
      </div>
    </form>
  );
}
