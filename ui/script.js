const invoke = window.__TAURI__.invoke;
const {open} = window.__TAURI__.dialog;

async function openImage() {
    const selected = await open({
        multiple: false,
        name: 'Image',
        extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'ico', 'tiff', 'pnm', 'dds', 'tga'],
    });
    if (selected !== null) {
        document.getElementById("sliderBinarizeThreshold").value = 128;
        document.getElementById("sliderSpeckleThreshold").value = 4;
        await generatePreview(selected);
        await generateSvg(selected);
    }
}

async function generateSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;

    invoke('generate_svg', {
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold
    }).then(svgData =>
        document.getElementById("imgSvgPreview").innerHTML = svgData
    );
}

async function saveSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;

    invoke('save_svg', {
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold
    });
}

async function generatePreview(imagePath) {
    invoke('generate_preview', {imagePath: imagePath});
}
