const invoke = window.__TAURI__.invoke;
const {open} = window.__TAURI__.dialog;

function enableControls() {
    document.getElementById("sliderSpeckleThreshold").value = 4;
    document.getElementById("sliderBinarizeThreshold").value = 128;
    document.getElementById("chkInvertBinary").checked = false;
    document.getElementById("chkColor").checked = false;
    document.getElementById("chkAlphaChannel").checked = false;
    document.getElementById("sliderColorCount").value = 5;
    document.getElementById("sliderGradientStep").value = 16;
    document.getElementById("optionsLayoutFieldset").style.visibility = "visible";
}

async function openImage() {
    const imagePath = await open({
        multiple: false,
        name: 'Image',
        extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'ico', 'tiff', 'avif', 'pnm', 'dds', 'tga'],
    });
    if (imagePath !== null) {
        enableControls();
        await loadImage(imagePath);
        await generateSvg();
    }
}

async function generateSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const invertBinary = document.getElementById("chkInvertBinary").checked;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;
    const colorCount = document.getElementById("sliderColorCount").value;
    const gradientStep = document.getElementById("sliderGradientStep").value;

    invoke('generate_svg', {
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold,
        invertBinary: invertBinary,
        colorCount: colorCount,
        gradientStep: gradientStep,
    }).then(svgData =>
        document.getElementById("imgSvgPreview").src = "data:image/svg+xml;utf8," + encodeURIComponent(svgData)
    );
}

async function changeState() {
    const withColor = document.getElementById("chkColor").checked;

    document.getElementById("labelColorCount").style.display = withColor ? "block" : "none";
    document.getElementById("sliderColorCount").style.display = withColor ? "block" : "none";

    document.getElementById("labelGradientStep").style.display = withColor ? "block" : "none";
    document.getElementById("sliderGradientStep").style.display = withColor ? "block" : "none";

    document.getElementById("labelBinarizeThreshold").style.display = withColor ? "none" : "block";
    document.getElementById("sliderBinarizeThreshold").style.display = withColor ? "none" : "block";

    document.getElementById("labelInvertBinary").style.display = withColor ? "none" : "block";
    document.getElementById("chkInvertBinary").style.display = withColor ? "none" : "block";

    document.getElementById("alphaChannelLayout").style.display = withColor ? "none" : "flex";

    invoke('color_state_changed', {
        withColor: withColor,
    });

    const ignoreAlphaChannel = document.getElementById("chkAlphaChannel").checked;

    invoke('alpha_channel_state_changed', {
        ignoreAlphaChannel: ignoreAlphaChannel,
    });

    await generateSvg();
}

async function saveSvg() {
    const binarizeThreshold = document.getElementById("sliderBinarizeThreshold").value;
    const invertBinary = document.getElementById("chkInvertBinary").checked;
    const speckleThreshold = document.getElementById("sliderSpeckleThreshold").value;
    const colorCount = document.getElementById("sliderColorCount").value;
    const gradientStep = document.getElementById("sliderGradientStep").value;

    invoke('save_svg', {
        speckleThreshold: speckleThreshold,
        binarizeThreshold: binarizeThreshold,
        invertBinary: invertBinary,
        colorCount: colorCount,
        gradientStep: gradientStep,
    });
}

async function loadImage(imagePath) {
    invoke('load_image', {imagePath: imagePath});
}
