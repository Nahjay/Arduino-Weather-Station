// Create Functions that will be used in the web interface
// These functions will be getting data from the server and displaying it on the web interface
// They will be implementations of get requests to the server for all of the buttons on the web interface
// This will be done using the fetch API 

// Function to get the current temperature from the server and display it on the web interface


// Get all buttons within the web interface
const buttons = document.querySelectorAll('button');

// Add click event listener to all buttons
buttons.forEach(button => {
    button.addEventListener('click', () => {
        const endpoint = button.id;
        const resultBoxId = `${endpoint.replace(/\//g, '_')}_box`;
        // fetchData(endpoint, resultBoxId);
        console.log(endpoint);
        console.log(resultBoxId);
    });
});

