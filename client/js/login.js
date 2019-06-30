// @ts-check

var BASE_URL = 'http://localhost:3000';

async function Login() {
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const resp = await makePostRequest('/auth/login', { username, password });
    if(!!resp.token) {
        alert(resp.token);
    }
}

async function makePostRequest(url, data) {
    return await fetch(`${BASE_URL}${url}`, {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
            'Content-Type': 'application/json'
        }
    }).then((res) => res.json())
    .then((res) => res)
    .catch((error) => {
        alert('something went wrong');
        console.error(error);
    })
}