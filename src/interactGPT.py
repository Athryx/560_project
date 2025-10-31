from openai import OpenAI
import os

# Create client (it will automatically pick up OPENAI_API_KEY from env vars)
client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))

messages = [
    {"role": "system", "content": "You are a proof assistant helping infer specifications for Verus Programs."}
]

print("Verus Proof Assistant (type 'exit' to quit)\n")

while True:
    message = input("Proof Query: ")
    if message.lower() in ["exit", "quit"]:
        print("Goodbye!")
        break

    messages.append({"role": "user", "content": message})

    try:
        chat = client.chat.completions.create(
            model="gpt-4-turbo",
            messages=messages
        )

        reply = chat.choices[0].message.content
        print(f"Proof Assistant: {reply}\n")

        messages.append({"role": "assistant", "content": reply})

    except Exception as e:
        print(f"Error: {e}")
