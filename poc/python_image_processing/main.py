import cv2 as cv
import numpy as np
import math


def get_levels(hierarchy):
    size = hierarchy.shape[1]
    level = np.full(size,-2)

    # set the level to 0 if the contour has no parent, set the level to one above the parent's if it does
    # this algorithm assumes that every parent goes before its children in the array
    for i in range(0, size):
        if hierarchy[0][i][3] == -1:
            level[i] = 0
        else:
            parent = hierarchy[0][i][3]
            level[i] = level[parent] + 1

    return level


def rainbow(input):
    collist = [(0, 0, 255), (0, 128, 255), (0, 230, 230), (0, 255, 0), (255, 0, 0), (255, 0, 255)]
    halved = math.floor(input/2)
    return collist[halved % len(collist)]



def highlight(image):
    im = image.copy()

    # convert the image to grayscale and threshold it before using findContours
    imgray = cv.cvtColor(im, cv.COLOR_BGR2GRAY)
    ret, thresh = cv.threshold(imgray, 127, 255, 0)

    contours, hierarchy = cv.findContours(thresh, cv.RETR_TREE, cv.CHAIN_APPROX_SIMPLE)

    output_image = np.full_like(image, 255)

    level = get_levels(hierarchy)

    for i in range(0, level.size):
        # ignore all even 'inside' edges, draw only odd 'outside' edges
        # opencv gives you both, but we only need one of the two
        if level[i] % 2 == 1:
            cv.drawContours(output_image, contours, i, rainbow(level[i]), 1)

    return output_image



def test():
    input = cv.imread("ar-for-education\\poc\\python_image_processing\\images\\rectified.png")

    result = highlight(input)
    cv.imshow("test", result)

    cv.waitKey(0)
    cv.destroyAllWindows()



if __name__ == '__main__':
    test()


