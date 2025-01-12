# IkigAI Backend

The backend for the **IkigAI** project, designed to help users discover their Ikigai using large language models (LLM).  

## About the Project

The project is inspired by the concept of "Ikigai" — a Japanese philosophy that translates to "reason for being."  

With the power of LLMs, users can receive personalized recommendations based on their unique interests, skills, and values.

## Key Technologies

- **Framework:** Django  
- **Language:** Python  
- **AI Models:** Integration with LLMs  
- **Database:** PostgreSQL  
- **Authentication:** JWT  

## Project Structure

```plaintext
ikigai-backend/
├── manage.py           # Django project management
├── ikigai/             # Core project settings
│   ├── settings.py     # Configuration
│   ├── urls.py         # URL routing
│   └── wsgi.py         # WSGI for deployment
├── apps/               # Django applications
│   ├── core/           # Core application
│   ├── auth/           # Authentication
│   └── recommendations/ # Recommendation logic
├── requirements.txt    # Project dependencies
└── README.md           # Documentation
