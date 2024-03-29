FROM ghcr.io/cross-rs/armv7-unknown-linux-musleabihf:latest
ARG DEBIAN_FRONTEND=noninteractive

RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install -y qemu-user-static

# At this point, you should be able to install armhf libraries and applications on your system and run them.
# We are going to build OpenCV with support for Python and C++. Let’s install NumPy and Python 2, 3 for the host system:
# tag::python[]
RUN apt-get install -y python3-dev
RUN apt-get install -y python3-numpy
RUN apt-get install -y python-dev
RUN apt-get install -y python-numpy

#We’ll also need libpython for the armhf architecture:
 RUN apt-get install -y libpython2-dev:armhf
 RUN apt-get install -y libpython3-dev:armhf
# end::python[]

# Other libraries requierd by OPENCV
## Getting the Armhf versions
# tag::open_deps[]
RUN apt-get install -y libtiff-dev:armhf zlib1g-dev:armhf
RUN apt-get install -y libjpeg-dev:armhf libpng-dev:armhf
RUN apt-get install -y libavcodec-dev:armhf libavformat-dev:armhf libswscale-dev:armhf libv4l-dev:armhf
RUN apt-get install -y libxvidcore-dev:armhf libx264-dev:armhf

# install the default cross compilers from Debian which can be used to create armhf binaries for Raspberry Pi:
RUN apt-get install -y crossbuild-essential-armhf
RUN apt-get install -y gfortran-arm-linux-gnueabihf

# Install CMAKe etc
RUN apt-get install -y cmake git pkg-config wget
# end::open_deps[]

# Intall OpenCV
# tag::opencv[]
WORKDIR /root
# <1>
RUN mkdir opencv_all && cd opencv_all \
    && wget -O opencv.tar.gz https://github.com/opencv/opencv/archive/4.5.5.tar.gz \
    && tar xf opencv.tar.gz \
    && wget -O opencv_contrib.tar.gz https://github.com/opencv/opencv_contrib/archive/4.5.5.tar.gz \
    && tar xf opencv_contrib.tar.gz \
    && rm *.tar.gz

# <2>
# variables required to successfully build GTK+
ENV PKG_CONFIG_PATH /usr/lib/arm-linux-gnueabihf/pkgconfig:/usr/share/pkgconfig
ENV PKG_CONFIG_LIBDIR /usr/lib/arm-linux-gnueabihf/pkgconfig:/usr/share/pkgconfig

# At this point, we can use Cmake to generate the OpenCV build scripts:
RUN cd /root/opencv_all/opencv-4.5.5 && mkdir build 

#RUN echo "$(ls /usr/lib/python3/dist-packages/numpy/core/include)"

# <3>
WORKDIR /root/opencv_all/opencv-4.5.5/build
RUN cmake -D CMAKE_BUILD_TYPE=RELEASE \
    -D CMAKE_INSTALL_PREFIX=/opt/opencv-4.5.5 \
    -D CMAKE_TOOLCHAIN_FILE=../platforms/linux/arm-gnueabi.toolchain.cmake \
    -D OPENCV_EXTRA_MODULES_PATH=~/opencv_all/opencv_contrib-4.5.5/modules \
    -D OPENCV_ENABLE_NONFREE=ON \
    -D ENABLE_NEON=ON \
    -D ENABLE_VFPV3=ON \
    -D BUILD_TESTS=OFF \
    -D BUILD_DOCS=OFF \
    -D PYTHON3_INCLUDE_PATH=/usr/include/python3.8 \
    -D PYTHON3_LIBRARIES=/usr/lib/arm-linux-gnueabihf/libpython3.8.so \
    -D PYTHON3_NUMPY_INCLUDE_DIRS=/usr/lib/python3/dist-packages/numpy/core/include \
    -D BUILD_OPENCV_PYTHON2=OFF \
    -D BUILD_OPENCV_PYTHON3=ON \
    -D BUILD_EXAMPLES=OFF \
    # Needed for the rust app
    -D OPENCV_GENERATE_PKGCONFIG=ON ..

# <4>
# Make the application
RUN make -j4
RUN make install/strip
# end::opencv[]

# tag::finish[]
# Change the name since it was mislabeled by the installer
# <1>
#RUN cd /opt/opencv-4.5.5/lib/python3.8/dist-packages/cv2/python-3.8/ \
#    && cp cv2.cpython-37m-x86_64-linux-gnu.so cv2.so

#RUN echo "$(ls /opt/opencv-4.5.5/lib/python3.8/site-packages/cv2/python-3.8/)"

RUN cd /opt/opencv-4.5.5/lib/python3.8/site-packages/cv2/python-3.8/ \
    && cp cv2.cpython-38-x86_64-linux-gnu.so cv2.so

# <2>
## Copy opencv-4.1.0-armhf.tar.bz2 and opencv.pc from your home folder to your Raspberry Pi.
RUN cd /opt \
    && tar -cjvf ~/opencv-4.5.5-armhf.tar.bz2 opencv-4.5.5

# <3>
## Creates a Pkg-config settings file
WORKDIR /root

# pkg-config settings file
RUN git clone https://gist.github.com/sol-prog/ed383474872958081985de733eaf352d opencv_cpp_compile_settings \
    && cd opencv_cpp_compile_settings \
    && cp opencv.pc ~

# Move the opencv.pc we copied in
RUN mv ~/opencv.pc /usr/lib/arm-linux-gnueabihf/pkgconfig

# <4>
# This is where our gnuebihf config will be
ENV PKG_CONFIG_PATH /usr/lib/arm-linux-gnueabihf/pkgconfig/
ENV OPENCV_PACKAGE_NAME /usr/lib/arm-linux-gnueabihf/pkgconfig/opencv.pc
ENV LD_LIBRARY_PATH /opt/opencv-4.5.5/lib
ENV PKG_CONFIG_ALLOW_CROSS 1
# end::finish[]

# Install SQLLite and developer items
#RUN apt-get install -y sqlite3:armhf
# tag::sql[]
RUN apt-get install -y libsqlite3-dev:armhf

# Install Open SSL needed for communication back to the UploadService
RUN apt-get install -y libssl-dev:armhf
# end::sql[]
# RUN apt-get -y install openssl
# RUN which openssl
# RUN ls -al /usr/bin/openssl
# ENV OPENSSL_DIR /usr/bin/openssl

