const toggleDarkMode = document.getElementById("toggleDarkMode");
toggleDarkMode.addEventListener("click", () => {
  document.body.classList.toggle("dark");
  // Update the button text based on the mode
  if (document.body.classList.contains("dark")) {
    toggleDarkMode.textContent = "Switch to Light Mode";
  } else {
    toggleDarkMode.textContent = "Switch to Dark Mode";
  }
});

function escapeHtml(unsafe) {
  return unsafe.replace(/[&<>"']/g, function (match) {
    const escapeChars = {
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      '"': "&quot;",
      "'": "&#x27;",
    };
    return escapeChars[match];
  });
}

function populateTable(emails) {
  const emailCardsContainer = document.getElementById("emailCardsContainer");

  emails.forEach((email) => {
    const card = document.createElement("div");
    card.classList.add(
      "card",
      "p-6",
      "shadow-lg",
      "rounded-xl",
      "overflow-hidden",
      "transform",
      "transition-all",
      "cursor-pointer"
    );

    const sender = escapeHtml(email.sender);
    const recipient = email.recipient.map((r) => escapeHtml(r)).join(", ");
    const subject = escapeHtml(email.subject);
    const body = escapeHtml(email.body);

    card.innerHTML = `
      <div class="flex items-center mb-4">
        <span class="bg-indigo-600 text-white rounded-full p-3 mr-3">
          <i class="fas fa-envelope"></i>
        </span>
        <div class="flex flex-col">
          <h3 class="subject hover:text-indigo-900">${subject}</h3>
          <p class="text-gray-500 text-sm">${sender} to ${recipient}</p>
        </div>
      </div>
      <p class="body text-gray-700 text-sm">${body.substring(0, 100)}...</p>
      <div class="mt-4 text-right">
        <span class="text-gray-500 text-xs">Received on ${new Date(
          email.date
        ).toLocaleString()}</span>
      </div>
    `;

    emailCardsContainer.appendChild(card);
  });
}

document.addEventListener("DOMContentLoaded", function () {
  fetch("../server/received_email.json")
    .then((response) => response.json())
    .then((data) => {
      populateTable(data);
      console.log(data);
    })
    .catch((error) => console.error("Error loading JSON:", error));
});
