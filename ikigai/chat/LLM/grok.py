from llama_index.llms.groq import Groq
from config import API_Grok


llm = Groq(model="llama3-70b-8192", api_key=API_Grok)

text = input(">>  ")
response = llm.complete(text)
print(response)