const invoke = window.__TAURI__.invoke;
const {open} = window.__TAURI__.dialog;

function enableControls() {
    document.getElementById("sliderBinarizeThreshold").value = 128;
    document.getElementById("sliderSpeckleThreshold").value = 4;
    document.getElementById("optionsLayoutFieldset").style.visibility = "visible";
}

async function openImage() {
    const selected = await open({
        multiple: false,
        name: 'Image',
        extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'ico', 'tiff', 'avif', 'pnm', 'dds', 'tga'],
    });
    if (selected !== null) {
        enableControls();
        await generatePreview(selected);
        await generateSvg();
    }
}

async function generateSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;
    const colorCount = document.getElementById("sliderColorCount").value;

    invoke('generate_svg', {
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold,
        colorCount: colorCount
    }).then(svgData =>
        document.getElementById("imgSvgPreview").innerHTML = svgData
    );
}

async function changeColorState() {
    const withColor = document.getElementById("chkColor").checked;
    document.getElementById("sliderColorCount").style.visibility = withColor ? "visible" : "collapse";
    document.getElementById("sliderBinarizeThreshold").style.visibility = withColor ? "collapse" : "visible";

    invoke('color_state_changed', {
        withColor: withColor,
    });

    await generateSvg();
}

async function saveSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;
    const colorCount = document.getElementById("sliderColorCount").value;

    invoke('save_svg', {
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold,
        colorCount: colorCount
    });
}

async function generatePreview(imagePath) {
    invoke('generate_preview', {imagePath: imagePath});
}
