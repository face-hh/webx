const express = require('express');
const app = express();
const PORT = 3000;

app.use(express.static('test'));

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});


