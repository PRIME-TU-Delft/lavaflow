import cv2 as cv
import numpy as np

import main


def getContours(image):
    im = image.copy()

    # convert the image to grayscale and threshold it before using findContours
    imgray = cv.cvtColor(im, cv.COLOR_BGR2GRAY)
    ret, thresh = cv.threshold(imgray, 127, 255, 0)

    contours, hierarchy = cv.findContours(thresh, cv.RETR_TREE, cv.CHAIN_APPROX_SIMPLE)

    thingy = np.full_like(image, 255)

    level = main.get_levels(hierarchy)

    result = []

    for i in range(0, level.size):
        # ignore all even 'inside' edges, draw only odd 'outside' edges
        # opencv gives you both, but we only need one of the two
        if level[i] % 2 == 1:
            cv.drawContours(thingy, contours, i, main.rainbow(level[i]), 1)


    return thingy


def binarize(img):
    asdf = img.copy()
    grayscale = cv.cvtColor(img, cv.COLOR_BGR2GRAY)
    # normalized = cv.normalize(grayscale, None, alpha=0, beta=200, norm_type=cv.NORM_MINMAX)
    ret,thresh = cv.threshold(grayscale,127,255,cv.THRESH_BINARY)
    # cv.imshow("thresh", thresh)
    closed = cv.morphologyEx(thresh, cv.MORPH_CLOSE, np.ones((5,5),np.uint8))
    # cv.imshow("closed", closed)

    return closed



def test():
    asdf = cv.imread("images\\rectified.png")

    result = getContours(asdf)

    cv.imshow("asdf", result)
    cv.waitKey(0)
    cv.destroyAllWindows()


if __name__ == '__main__':
    test()
