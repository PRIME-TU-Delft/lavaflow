#include <vector>
using namespace std;
#include "Constructor.h"


//#include findclosestpoint
//#include calcinv weighted average


        int rasterSize;
        int contourMargin;
        int xCells;
        int yCells;

        int xDimension;
        int yDimenstion;

        vector<vector<bool>> isSVC;
        vector<vector<float>> knownHeights;
        
         vector<vector<vector<float>>> contourLines; 
         vector<vector<vector<float>>> knownHeights;


        void initialize() {
                  //calculate total number of cells in x and y dir


                  //TODO make dimensions range class

            // cellsX = ceil((xDimensions.max  - xDimensions.min ) / rasterSize);
            // cellsY = ceil((yDimensions.max  - yDimensions.min ) / rasterSize);


            //todo use : 
           // fill(vect1.begin(), vect1.end(), value);


            //set starting values for isSVC and Knownheights 
            for (int i  = 0 ; i< xCells ; i++){

                for (int j  = 0 ; j< yCells ; j++){

                        // edges of map should have height 0
                        if(i == 0 || j == 0 || i == xCells - 1 || j == yCells -1){
                            isSVC[i][j] = true;
                            knownHeights[i][j] = 0;
                        }else {
                            knownHeights[i][j] = -1;
                            isSVC[i][j] = false;
                        }
                        
                };
            };
        }


        bool checkSVC(vector<float> p, int  ix, int iy );
        float localTIN(vector<float> p);
        bool calcheigtsNVC();



