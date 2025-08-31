import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import RectangleSelector
import cv2

GAMMA = 1.8  # Range [1.8,2.4]

def srgb_to_linear(x: np.ndarray) -> np.ndarray:
    # Based on https://www.w3.org/Graphics/Color/srgb 8.B.1
    a = 0.055
    return np.where(
        x <= 0.04045,
        x / 12.92,
        ((x + a) / (1 + a)) ** GAMMA
    )

def linear_to_srgb(x: np.ndarray) -> np.ndarray:
    # Based on https://www.w3.org/Graphics/Color/srgb 8.B
    a = 0.055
    return np.where(
        x <= 0.0031308,
        12.92 * x,
        (1 + a) * np.power(x, 1/GAMMA) - a
    )

class NegativeConverter:
    def __init__(self):
        self.film_base_coordinates = None
        self.film_base_estimate = None

    def _selection_callback(self, event_click, event_release):
        x1, y1 = int(event_click.xdata), int(event_click.ydata)
        x2, y2 = int(event_release.xdata), int(event_release.ydata)
        # stored as (y0,y1,x0,x1) of the rectangular selection
        self.film_base_coordinates = (min(y1, y2), max(y1, y2), min(x1, x2), max(x1, x2))

    def select_film_base(self, img_rgb: np.ndarray):
        fig, ax = plt.subplots()
        ax.imshow(img_rgb)
        ax.set_title("Select a rectangular region that correspond to the film's base \nand then close the window")
        _ = RectangleSelector(
            ax, self._selection_callback,
            useblit=True,
            minspanx=5, minspany=5,
            spancoords='pixels', interactive=True
        )
        plt.show(block=True)

        if self.film_base_coordinates is None:
            raise RuntimeError("Film base patch not selected")
        else:
            y0, y1, x0, x1 = self.film_base_coordinates
            patch = img_rgb[y0:y1, x0:x1, :]
            patch_lin = srgb_to_linear(patch.astype(np.float32) / 255.0)
            self.film_base_estimate = patch_lin.mean(axis=(0, 1))
            print("Film base estimate:", self.film_base_estimate)

    def invert(self, img_rgb: np.ndarray) -> np.ndarray:
        lin = srgb_to_linear(img_rgb.astype(np.float32) / 255.0)

        if self.film_base_estimate is None:
            self.select_film_base(img_rgb)

        # Scene bright → Film is dense → Darker scanned image
        # Scene dark → Film is clear → Brighter scanned image
        # so, film_base_estimate is the brightest possible value (unexposed film, therefore clear film)
        inv = np.clip((self.film_base_estimate - lin) / self.film_base_estimate, 0, 1)
        srgb = linear_to_srgb(inv)
        return (srgb * 255).astype(np.uint8)

def main():
    input_path = r"negative.jpg"
    output_path = r"positive.jpg"

    bgr = cv2.imread(input_path, cv2.IMREAD_COLOR)
    if bgr is None:
        raise FileNotFoundError(f"Failed to open {input_path}")
    rgb = cv2.cvtColor(bgr, cv2.COLOR_BGR2RGB)
    print(f"Loaded negative image from {input_path}")

    converter = NegativeConverter()
    positive = converter.invert(rgb)

    cv2.imwrite(output_path, cv2.cvtColor(positive, cv2.COLOR_RGB2BGR))
    print(f"Saved positive image to {output_path}")

    fig, axes = plt.subplots(1, 2, figsize=(12, 6))
    axes[0].imshow(rgb)
    axes[0].set_title("Negative")
    axes[0].axis("off")
    axes[1].imshow(positive)
    axes[1].set_title("Positive")
    axes[1].axis("off")
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    main()
