from config import PATH_gpt4all

from gpt4all import GPT4All # type: ignore
model = GPT4All(PATH_gpt4all)

name = input("What is your name?\n>>  ")
with model.chat_session():
    while True:
        user_input = input(f"\n{name}: ")
        response = model.generate(user_input)
        print(response)
