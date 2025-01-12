from config import API_chatGPT

LANGSMITH_TRACING=True
LANGSMITH_ENDPOINT="https://api.smith.langchain.com"
LANGSMITH_API_KEY="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"
LANGSMITH_PROJECT="pr-reflecting-placebo-81"
OPENAI_API_KEY=API_chatGPT


from langchain_openai import ChatOpenAI

llm = ChatOpenAI()
llm.invoke("Hello, world!")