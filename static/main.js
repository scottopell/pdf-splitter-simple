import init, { PdfProcessor } from '../pkg/pdf_splitter_simple.js';

async function run() {
    await init();
    const processor = new PdfProcessor();

    document.getElementById('process').addEventListener('click', async () => {
        const fileInput = document.getElementById('upload');
        const file = fileInput.files[0];
        if (file) {
            try {
                const metadata = await processor.process_pdf(file);
                displayMetadata(metadata);
            } catch (err) {
                console.error("Error processing PDF:", err);
            }
        }
    });
}

function displayMetadata(metadata) {
    const metadataObj = JSON.parse(metadata);
    const output = document.getElementById('output');
    output.innerHTML = `
        <h2>PDF Metadata</h2>
        <p><strong>Title:</strong> ${metadataObj.title}</p>
        <p><strong>Number of Pages:</strong> ${metadataObj.num_pages}</p>
        <h3>Images</h3>
        ${metadataObj.images.map(img => `<img src="data:image/png;base64,${img}" alt="PDF Page" />`).join('')}
    `;
}

run();