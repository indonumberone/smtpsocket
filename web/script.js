const emails = [
  {
    from: "muqsith@yahoo.com",
    to: "mitaaa@gmail.com",
    subject: "Meeting Schedule",
    message: "jbegkeeboqueb",
    date: "2024-12-03/21.50 PM",
  },
  {
    from: "kinderjoy@awokawok.com",
    to: "prabow@merdeka.com",
    subject: "Project Update",
    message: "jebgqebqeuq.",
    date: "2024-12-03/21.50 PM",
  },
];

function populateTable() {
  const emailTableBody = document.getElementById("emailTableBody");

  emails.forEach((email) => {
    const row = document.createElement("tr");

    row.innerHTML = `
          <td class="px-4 py-2">${email.from}</td>
          <td class="px-4 py-2">${email.to}</td>
          <td class="px-4 py-2">${email.subject}</td>
          <td class="px-4 py-2">${email.message}</td>
          <td class="px-4 py-2">${email.date}</td>
      `;

    emailTableBody.appendChild(row);
  });
}

document.addEventListener("DOMContentLoaded", populateTable);
