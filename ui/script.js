const invoke = window.__TAURI__.invoke;
const {open} = window.__TAURI__.dialog;

var imagePath = null;

async function openImage() {
    const selected = await open({
        multiple: false,
        name: 'Image',
        extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'ico', 'tiff', 'pnm', 'avif', 'dds', 'tga'],
    });
    if (selected !== null) {
        imagePath = selected;
        await generateSvg();
    }
}

async function generateSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;

    invoke('generate_svg', {
        imagePath: imagePath,
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold
    }).then((svgPath) =>
        document.getElementById("imgSvgPreview").src = "asset://" + svgPath + "?x=" + new Date().getTime()
    );
}