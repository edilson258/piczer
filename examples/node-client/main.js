const fs = require('fs');

async function uploadImageToResize(imagePath, newWidth, newHeight, outImageName) {
  const imageBuffer = fs.readFileSync(imagePath);
  const serverUrl = `http://127.0.0.1:8000/?dim=${newWidth}x${newHeight}`;

  const headers = new Headers({
    'Content-Type': 'application/octet-stream',
  });

  const request = new Request(serverUrl, {
    method: 'POST',
    headers: headers,
    body: imageBuffer,
  });

  try {
    const response = await fetch(request);

    if (response.status === 200) {
      const imageBinaryData = await response.arrayBuffer();
      writeBinaryDataToFile(imageBinaryData, outImageName);
    } else {
      const text = await response.text();
      throw new Error(text);
    }
  } catch (err) {
    console.error(err);
  }
}

function writeBinaryDataToFile(binaryData, outImageName) {
  const buffer = Buffer.from(binaryData);

  fs.writeFile(outImageName, buffer, (err) => {
    if (err) {
      console.error(err);
    } else {
      console.log("Resized image saved successfully!");
    }
  });
}

uploadImageToResize("./image.jpg", 2,  2, "resized.png");
