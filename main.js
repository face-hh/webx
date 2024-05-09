const express = require('express');
const app = express();
const PORT = 3000;

var bodyParser = require('body-parser')
app.use( bodyParser.json() );       // to support JSON-encoded bodies
app.use(bodyParser.urlencoded({     // to support URL-encoded bodies
  extended: true
})); 
app.use(express.json());       // to support JSON-encoded bodies
app.use(express.urlencoded()); // to support URL-encoded bodies


app.post('/', (req, res) => {
  console.log('Headers:', req.headers);
  console.log('Body:', req.body);
  res.json({ message: 'Hello, world!' });
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});


