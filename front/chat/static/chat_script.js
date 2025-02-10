document.getElementById('send-button').addEventListener('click', function() {
    const userInput = document.getElementById('user-input').value;
    if (userInput.trim() !== '') {
        displayMessage(userInput, 'user');
        document.getElementById('user-input').value = '';

        // Simulate bot's response (You can replace this with an API call for real interaction)
        setTimeout(() => {
            const botResponse = 'Привет! Как я могу помочь?';  // Example front response
            displayMessage(botResponse, 'bot');
        }, 1000);
    }
});

document.getElementById('user-input').addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
        document.getElementById('send-button').click();
    }
});

function displayMessage(message, sender) {
    const messageElement = document.createElement('div');
    messageElement.classList.add('chat-message');
    messageElement.classList.add(sender);
    messageElement.textContent = message;

    const chatBox = document.getElementById('chat-box');
    chatBox.appendChild(messageElement);
    chatBox.scrollTop = chatBox.scrollHeight;  // Auto scroll to the bottom
}
