from llama_index.llms.llama_api import LlamaAPI
from config import API_llama


llm = LlamaAPI(api_key=API_llama)

text = input(">> ")
resp = llm.complete(text)
print(resp)

