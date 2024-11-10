export async function uploadToPinata(apiKey, apiSecret, data) {
    const formData = new FormData();
    const blob = new Blob([JSON.stringify(data)], { type: 'application/json' });
    formData.append('file', blob, 'metadata.json');

    const response = await fetch('https://api.pinata.cloud/pinning/pinFileToIPFS', {
        method: 'POST',
        headers: {
            'pinata_api_key': apiKey,
            'pinata_secret_api_key': apiSecret,
        },
        body: formData
    });

    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
} 