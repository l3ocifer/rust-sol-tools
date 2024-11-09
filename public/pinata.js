export async function uploadToPinata(apiKey, apiSecret, data) {
    const url = 'https://api.pinata.cloud/pinning/pinFileToIPFS';
    
    let formData = new FormData();
    if (data instanceof File) {
        formData.append('file', data);
    } else {
        const blob = new Blob([JSON.stringify(data)], { type: 'application/json' });
        formData.append('file', blob, 'metadata.json');
    }

    const response = await fetch(url, {
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

    const result = await response.json();
    return `https://gateway.pinata.cloud/ipfs/${result.IpfsHash}`;
} 