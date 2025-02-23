
document.addEventListener("DOMContentLoaded", function() {
    const title = document.querySelector("h2");
    title.addEventListener("click", function() {
        this.textContent = "Вы нажали на заголовок!";
    });
});
