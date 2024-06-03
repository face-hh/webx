const crypto = require('crypto');

function generateSecretKey(length) {
    const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let key = '';

    for (let i = 0; i < length; i++) {
        const randomIndex = Math.floor(crypto.randomBytes(4).readUInt32LE(0)/0xffffffff * charset.length);
        key += charset.charAt(randomIndex);
    }

    return key;
}

module.exports = {
    generateSecretKey
}
