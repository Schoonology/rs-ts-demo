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
      <label htmlFor="message">
        <input
          className="p-3"
          type="text"
          name="message"
          placeholder="Message"
          value={message}
          onChange={(event) => setMessage(event.target.value)}
        />
      </label>
      <button type="submit">Submit</button>
    </form>
  );
}
