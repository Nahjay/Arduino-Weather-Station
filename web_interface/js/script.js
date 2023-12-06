// Create Functions that will be used in the web interface
// These functions will be getting data from the server and displaying it on the web interface
// They will be implementations of get requests to the server for all of the buttons on the web interface
// This will be done using the fetch API which is a modern replacement for XMLHttpRequest


// Get all buttons within the web interface
const buttons = document.querySelectorAll('button');

// Add click event listener to all buttons
buttons.forEach(button => {
    button.addEventListener('click', () => {
        const endpoint = button.id;
        const resultBoxId = `${endpoint.replace(/\//g, '_')}_box`;
        fetchData(endpoint, resultBoxId);
        console.log(endpoint);
        console.log(resultBoxId);
    });
});

function fetchData(endpoint, resultBoxId) {
    fetch(`http://localhost:8084${endpoint}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json(); // assuming the response is in JSON format
        })
        .then(data => {
            // Handle the retrieved data and update the respective result box
            document.getElementById(resultBoxId).innerText = JSON.stringify(data);
        })
        .catch(error => {
            // Handle errors
            console.error('There was a problem with the fetch operation:', error);
        });
} 
